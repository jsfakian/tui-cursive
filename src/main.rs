use anyhow::Result;
use cursive::{
    event::{Event, EventResult, Key},
    reexports::log::{self},
    traits::{Nameable, Resizable},
    views::{Dialog, LinearLayout, ListView, OnEventView, SelectView, TextView},
    CursiveRunnable, CursiveRunner,
};
use std::{cell::RefCell, rc::Rc, sync::mpsc};

#[derive(Debug)]
pub struct BlkDevice {
    #[allow(dead_code)]
    partitions: Option<Vec<BlkDevice>>,
    name: String,
}

impl BlkDevice {
    #[must_use]
    fn new<S>(name: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            partitions: None,
            name: name.into(),
        }
    }

    #[must_use]
    fn with_partitions<S, T>(name: S, partitions: T) -> Self
    where
        S: Into<String>,
        T: IntoIterator,
        T::Item: Into<BlkDevice>,
    {
        Self {
            partitions: Some(partitions.into_iter().map(|e| e.into()).collect()),
            name: name.into(),
        }
    }
}

//#[derive(Debug)]
struct Controller {
    devices: Rc<RefCell<Vec<BlkDevice>>>,
    on_init: bool,
    rx: mpsc::Receiver<ControllerMessage>,
    ui: Ui,
}

impl Controller {
    #[must_use]
    fn new() -> Result<Self> {
        let (tx, rx) = mpsc::channel::<ControllerMessage>();

        Ok(Self {
            devices: Rc::new(RefCell::new(vec![
                BlkDevice::new("/dev/sda"),
                BlkDevice::with_partitions(
                    "/dev/sdb",
                    [
                        BlkDevice::new("/dev/sdb1"),
                        BlkDevice::new("/dev/sdb2"),
                        BlkDevice::new("/dev/sdb3"),
                    ],
                ),
                BlkDevice::new("/dev/sdc"),
            ])),
            on_init: true,
            rx: rx,
            ui: Ui::new(tx.clone()),
        })
    }

    /// Run the controller
    pub fn run(&mut self) {
        cursive::logger::init();
        log::set_max_level(log::LevelFilter::Info);

        if self.on_init {
            self.ui
                .cursive
                .add_global_callback('~', cursive::Cursive::toggle_debug_console);
            self.on_init = false;
            self.ui.send(UiMessage::ShowDiskList(self.devices.clone()));
        }

        self.ui.refresh();

        while self.ui.step() {
            while let Some(message) = self.rx.try_iter().next() {
                // Handle messages arriving from the UI.
                match message {
                    ControllerMessage::UpdatePartitions => {
                        self.devices.borrow_mut().push(BlkDevice::new("/dev/sdу"));
                        self.ui.send(UiMessage::ShowDiskList(self.devices.clone()));
                        log::info!("Disk added");
                    }
                    ControllerMessage::InstallOn(_) => todo!(),
                };
            }
        }
    }
}

pub enum ControllerMessage {
    UpdatePartitions,
    InstallOn(usize),
}

pub struct Ui {
    cursive: CursiveRunner<CursiveRunnable>,
    ui_rx: mpsc::Receiver<UiMessage>,
    ui_tx: mpsc::Sender<UiMessage>,
    #[allow(dead_code)]
    controller_tx: mpsc::Sender<ControllerMessage>,
}

type ListItemData = (usize, usize);
type DiskSelectView = OnEventView<SelectView<ListItemData>>;

impl Ui {
    /// Create a new Ui object.  The provided `mpsc` sender will be used
    /// by the UI to send messages to the controller.
    pub fn new(controller_tx: mpsc::Sender<ControllerMessage>) -> Ui {
        let (ui_tx, ui_rx) = mpsc::channel::<UiMessage>();
        let mut ui = Ui {
            cursive: cursive::default().into_runner(),
            ui_tx: ui_tx,
            ui_rx: ui_rx,
            controller_tx: controller_tx,
        };
        let header = LinearLayout::horizontal()
            .child(TextView::new("Disk").min_width(20))
            .child(TextView::new("Size"));

        let select_view = SelectView::<ListItemData>::new().on_submit(|_list, _item| {});

        let select_view = OnEventView::new(select_view)
            .on_pre_event_inner(Event::Key(Key::Down), |list, _evt| {
                list.selection().and_then(|current| {
                    let cb = list.select_down(current.1 + 1);
                    Some(EventResult::Consumed(Some(cb)))
                })
            })
            .on_pre_event_inner(Event::Key(Key::Up), |list, _evt| {
                list.selection().and_then(|current| {
                    let mut scroll_size = 1;
                    if current.0 != 0 {
                        if let Some(prev) = list.get_item(current.0 - 1) {
                            scroll_size = prev.1 .1;
                        }
                    }
                    let cb = list.select_up(scroll_size + 1);
                    Some(EventResult::Consumed(Some(cb)))
                })
            })
            .on_pre_event(Event::Key(Key::PageUp), |_| {})
            .on_pre_event(Event::Key(Key::PageDown), |_| {})
            .with_name("InstallDiskList");

        let select_dialog = Dialog::new()
            .title("Select installation disk")
            .button("Exit", |siv| siv.add_layer(Ui::create_exit_dialog()))
            .content(ListView::new().child("", header).child("", select_view))
            .with_name("DiskSelectDialog");

        ui.cursive.add_layer(select_dialog);

        // set initial focus to disk selection list
        ui.cursive.focus_name("InstallDiskList").unwrap();

        // handle Ctrl+C
        ui.cursive.clear_global_callbacks(Event::CtrlChar('c'));
        ui.cursive.set_on_pre_event(Event::CtrlChar('c'), |siv| {
            //TODO: check if the top layer is not an Exit dialog
            siv.add_layer(Ui::create_exit_dialog());
        });

        //TODO: probably this is not a good idea and some views shouldn't be dismissed
        //TODO: by ESC key, but this is just for reference
        ui.cursive.set_global_callback(Event::Key(Key::Esc), |siv| {
            if siv.screen().len() > 1 {
                siv.pop_layer();
            } else {
                siv.add_layer(Ui::create_exit_dialog());
            }
        });

        ui
    }

    fn create_exit_dialog() -> Dialog {
        Dialog::text("Do you want to exit?\nThe device will be rebooted")
            .button("Yes", |siv| siv.quit())
            .button("No", |siv| {
                siv.pop_layer();
            })
    }

    pub fn refresh(&mut self) {
        self.cursive.refresh();
    }

    fn redraw(&mut self) {
        self.cursive
            .cb_sink()
            .send(Box::new(cursive::Cursive::noop))
            .unwrap();
    }

    pub fn send(&mut self, msg: UiMessage) {
        self.ui_tx.send(msg).unwrap();
        self.redraw();
    }
    /// Step the UI by calling into Cursive's step function, then
    /// processing any UI messages.
    pub fn step(&mut self) -> bool {
        if !self.cursive.is_running() {
            return false;
        }

        // Process any pending UI messages
        while let Some(message) = self.ui_rx.try_iter().next() {
            match message {
                UiMessage::ShowDiskList(disks) => {
                    let mut list = self
                        .cursive
                        .find_name::<DiskSelectView>("InstallDiskList")
                        .unwrap();
                    let list = list.get_inner_mut();
                    list.clear();
                    for (i, disk) in disks.borrow().iter().enumerate() {
                        let s = format!("{disk:<20}{size}", disk = &disk.name, size = "10G");
                        if let Some(parts) = &disk.partitions {
                            list.add_item(&s, (i, parts.len()));

                            let mut it = parts.iter().peekable();
                            while let Some(part) = it.next() {
                                let symbol = if it.peek().is_none() {
                                    '\u{2514}'
                                } else {
                                    '\u{251C}'
                                };
                                let s = format!(" {}\u{2500}{}", symbol, part.name);
                                list.add_item(&s, (0, 0));
                            }
                        } else {
                            list.add_item(&s, (i, 0));
                        }
                    }

                    list.set_on_submit(|siv, item| {
                        siv.add_layer(Dialog::text(format!("Item {} selected", item.0)).button(
                            "Ok",
                            |siv| {
                                siv.pop_layer();
                            },
                        ))
                    });

                    // TODO: keep this code as a reference for future use
                    // let tx = self.controller_tx.clone();
                    // list.set_on_select(move |_e, l| {
                    //     tx.send(ControllerMessage::UpdatePartitions).unwrap();
                    // });
                }
            }
        }
        self.cursive.step();
        true
    }
}

pub enum UiMessage {
    ShowDiskList(Rc<RefCell<Vec<BlkDevice>>>),
}

fn main() -> Result<()> {
    // Launch the controller and UI
    let controller = Controller::new();
    println!("hello 1");
    match controller {
        Ok(mut controller) => controller.run(),
        Err(e) => println!("Error: {}", e),
    };
    Ok(())
}
