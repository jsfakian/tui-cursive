use anyhow::{anyhow, Result};
use cursive::{
    event::{Event, EventResult, Key},
    reexports::log::{self},
    traits::{Nameable, Resizable},
    views::{Dialog, LinearLayout, ListView, OnEventView, SelectView, TextView},
    CursiveRunnable, CursiveRunner,
};
use std::{
    cell::RefCell,
    rc::Rc,
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc,
    },
    thread::{self, JoinHandle},
};
mod fsm;
use fsm::{FsmMsgHelper, State, StateName, Transition, FSM};

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

struct MsgHelper {
    tx_ui: Sender<ControllerMessage>,
    tx_self: Sender<ControllerMessage>,
}

impl FsmMsgHelper<ControllerMessage> for MsgHelper {
    fn send_to_self(&mut self, msg: ControllerMessage) -> Result<()> {
        self.tx_self.send(msg)?;
        Ok(())
    }

    fn send_to_ui(&mut self, msg: ControllerMessage) -> Result<()> {
        self.tx_ui.send(msg)?;
        Ok(())
    }

    fn pre_handle_message(
        &mut self,
        msg: &ControllerMessage,
    ) -> Result<Transition<ControllerMessage>> {
        Ok(fsm::Transition::DoNothing)
    }
}
struct Controller {
    tx: mpsc::Sender<ControllerMessage>,
    ui: Ui,
    thread_handle: Option<JoinHandle<Result<(), anyhow::Error>>>,
}

struct GetBlockDevices {
    disks: Option<Arc<Vec<BlkDevice>>>,
}
impl State<ControllerMessage> for GetBlockDevices {
    fn on_enter(&mut self, fsm: &mut dyn FsmMsgHelper<ControllerMessage>) -> Result<()> {
        println!("{}.on_enter()", self.name());
        log::info!("{}.on_enter()", self.name());
        let devices = vec![
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
        ];
        self.disks = Some(Arc::new(devices));
        fsm.send_to_self(ControllerMessage::Next);
        Ok(())
    }

    fn on_event(
        &mut self,
        fsm: &mut dyn FsmMsgHelper<ControllerMessage>,
        msg: ControllerMessage,
    ) -> Result<fsm::Transition<ControllerMessage>> {
        println!("{}.on_event({:?})", self.name(), msg);
        log::info!("{}.on_event({:?})", self.name(), msg);
        let t = match msg {
            ControllerMessage::Next => {
                let disks = self
                    .disks
                    .take()
                    .ok_or_else(|| anyhow!("Disks are empty"))?;
                fsm::Transition::ChangeState(Box::new(SelectInstallDisk { disks: disks }))
            }
            _ => {
                log::error!("Unexpected: {:?}", msg);
                fsm::Transition::DoNothing
            }
        };
        log::info!("tr: {:?}", t);
        Ok(t)
    }
}

struct BlkDeviceInfo {
    name: String,
    partitions: Option<Vec<BlkDeviceInfo>>,
    size: usize,
}
struct SelectInstallDiskUI {
    disks: Vec<BlkDeviceInfo>,
}
impl CreateUi for SelectInstallDiskUI {
    fn create_ui(&self, tx: &Sender<ControllerMessage>) -> Box<dyn cursive::View> {
        let header = LinearLayout::horizontal()
            .child(TextView::new("Disk").min_width(20))
            .child(TextView::new("Size"));

        let tx = tx.clone();

        let mut select_view = SelectView::<ListItemData>::new().on_submit(move |_list, _item| {
            tx.send(ControllerMessage::DiskSelected(_item.1)).unwrap()
        });

        for (i, disk) in self.disks.iter().enumerate() {
            let s = format!("{disk:<20}{size}", disk = &disk.name, size = "10G");
            if let Some(parts) = &disk.partitions {
                select_view.add_item(&s, (i, parts.len()));

                let mut it = parts.iter().peekable();
                while let Some(part) = it.next() {
                    let symbol = if it.peek().is_none() {
                        '\u{2514}'
                    } else {
                        '\u{251C}'
                    };
                    let s = format!(" {}\u{2500}{}", symbol, part.name);
                    select_view.add_item(&s, (0, 0));
                }
            } else {
                select_view.add_item(&s, (i, 0));
            }
        }

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

        Box::new(select_dialog)
    }
}
struct SelectInstallDisk {
    disks: Arc<Vec<BlkDevice>>,
}
impl State<ControllerMessage> for SelectInstallDisk {
    fn on_event(
        &mut self,
        fsm: &mut dyn FsmMsgHelper<ControllerMessage>,
        msg: ControllerMessage,
    ) -> Result<Transition<ControllerMessage>> {
        println!("{}.on_event({:?})", self.name(), msg);
        Ok(Transition::DoNothing)
    }

    fn on_enter(&mut self, fsm: &mut dyn FsmMsgHelper<ControllerMessage>) -> Result<()> {
        println!("{}.on_enter()", self.name());
        log::info!("{}.on_enter()", self.name());
        // if we do not have disk in config then show the ui

        let infos = self
            .disks
            .iter()
            .map(|e| {
                let mut dev = BlkDeviceInfo {
                    name: e.name.clone(),
                    partitions: None,
                    size: 1000000,
                };
                dev.partitions = e.partitions.as_ref().and_then(|e| {
                    Some(
                        e.iter()
                            .map(|e| BlkDeviceInfo {
                                name: e.name.clone(),
                                partitions: None,
                                size: 200000,
                            })
                            .collect(),
                    )
                });
                dev
            })
            .collect();

        fsm.send_to_ui(ControllerMessage::ShowUi(Box::new(SelectInstallDiskUI {
            disks: infos,
        })))
        .unwrap();
        Ok(())
    }
}

impl Controller {
    #[must_use]
    fn new() -> Result<Self> {
        let (tx, rx) = mpsc::channel::<ControllerMessage>();

        let ui = Ui::new(tx.clone());
        let ui_tx_clone = ui.ui_tx.clone();
        let tx_clone = tx.clone();

        let handle = thread::spawn(move || -> Result<()> {
            log::info!("Thread started");
            let mut fsm = FSM::new(
                Box::new(GetBlockDevices { disks: None }),
                MsgHelper {
                    tx_ui: ui_tx_clone,
                    tx_self: tx_clone,
                },
            );

            while let Ok(msg) = rx.recv() {
                log::info!("Got controller message: {:?}", msg);
                match msg {
                    ControllerMessage::Start => fsm.start()?,
                    _ => fsm.on_event(msg)?,
                }
            }
            Ok(())
        });

        Ok(Self {
            tx: tx, // can sent to self
            ui: ui,
            thread_handle: Some(handle),
        })
    }

    fn start(&mut self) {
        self.tx.send(ControllerMessage::Start).unwrap();
        self.ui.refresh();
    }

    /// Run the controller
    pub fn run(&mut self) {
        cursive::logger::init();
        log::set_max_level(log::LevelFilter::Info);
        log::info!("run() ->>");

        self.ui
            .cursive
            .add_global_callback('~', cursive::Cursive::toggle_debug_console);

        self.start();

        while self.ui.step() {
            //TODO: how to get rid of this loop???
        }
    }
}

trait CreateUi: Send + Sync {
    fn create_ui(&self, tx: &Sender<ControllerMessage>) -> Box<dyn cursive::View>;
}

//#[derive(Debug)]
enum ControllerMessage {
    Start,
    Next,
    ShowUi(Box<dyn CreateUi>),
    DiskSelected(usize),
}

impl std::fmt::Debug for ControllerMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Start => write!(f, "Start"),
            Self::ShowUi(_arg0) => write!(f, "ShowUi"), //f.debug_tuple("ShowUi").field(arg0).finish(),
            Self::Next => write!(f, "Next"),
            Self::DiskSelected(arg0) => f.debug_tuple("DiskSelected").field(arg0).finish(),
        }
    }
}

pub struct Ui {
    cursive: CursiveRunner<CursiveRunnable>,
    ui_rx: mpsc::Receiver<ControllerMessage>,
    ui_tx: mpsc::Sender<ControllerMessage>,
    #[allow(dead_code)]
    controller_tx: mpsc::Sender<ControllerMessage>,
}

type ListItemData = (usize, usize);
type DiskSelectView = OnEventView<SelectView<ListItemData>>;

impl Ui {
    /// Create a new Ui object.  The provided `mpsc` sender will be used
    /// by the UI to send messages to the controller.
    fn new(controller_tx: mpsc::Sender<ControllerMessage>) -> Ui {
        let (ui_tx, ui_rx) = mpsc::channel::<ControllerMessage>();
        let ui = Ui {
            cursive: cursive::default().into_runner(),
            ui_tx: ui_tx,
            ui_rx: ui_rx,
            controller_tx: controller_tx,
        };

        // // set initial focus to disk selection list
        // ui.cursive.focus_name("InstallDiskList").unwrap();

        // // handle Ctrl+C
        // ui.cursive.clear_global_callbacks(Event::CtrlChar('c'));
        // ui.cursive.set_on_pre_event(Event::CtrlChar('c'), |siv| {
        //     //TODO: check if the top layer is not an Exit dialog
        //     siv.add_layer(Ui::create_exit_dialog());
        // });

        // //TODO: probably this is not a good idea and some views shouldn't be dismissed
        // //TODO: by ESC key, but this is just for reference
        // ui.cursive.set_global_callback(Event::Key(Key::Esc), |siv| {
        //     if siv.screen().len() > 1 {
        //         siv.pop_layer();
        //     } else {
        //         siv.add_layer(Ui::create_exit_dialog());
        //     }
        // });

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

    // pub fn send(&mut self, msg: UiMessage) {
    //     self.ui_tx.send(msg).unwrap();
    //     self.redraw();
    // }
    /// Step the UI by calling into Cursive's step function, then
    /// processing any UI messages.
    pub fn step(&mut self) -> bool {
        if !self.cursive.is_running() {
            return false;
        }

        // Process any pending UI messages
        while let Some(message) = self.ui_rx.try_iter().next() {
            log::info!("got ui message {:?}", message);
            match message {
                ControllerMessage::ShowUi(e) => {
                    let ui = e.create_ui(&self.controller_tx);
                    self.cursive.add_layer(ui);
                    self.redraw();
                }
                _ => panic!(),
            }
        }
        self.cursive.step();
        true
    }
}

pub enum UiMessage {
    ShowDiskList(Vec<BlkDevice>),
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
