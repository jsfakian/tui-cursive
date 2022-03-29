use anyhow::Result;
use cursive::{
    reexports::log::{self},
    traits::{Nameable, Resizable},
    views::{Dialog, LinearLayout, ListView, SelectView, TextView},
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
                        BlkDevice::new("/dev/sda"),
                        BlkDevice::new("/dev/sda"),
                        BlkDevice::new("/dev/sda"),
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
                    ControllerMessage::UpdatedInputAvailable(text) => {
                        self.ui.ui_tx.send(UiMessage::UpdateOutput(text)).unwrap();
                    }
                    ControllerMessage::UpdatePartitions => {
                        self.devices.borrow_mut().push(BlkDevice::new("/dev/sdу"));
                        self.ui.send(UiMessage::ShowDiskList(self.devices.clone()));
                        log::info!("Disk added");
                    }
                };
            }
        }
    }
}

pub enum ControllerMessage {
    UpdatedInputAvailable(String),
    UpdatePartitions,
}

pub struct Ui {
    cursive: CursiveRunner<CursiveRunnable>,
    ui_rx: mpsc::Receiver<UiMessage>,
    ui_tx: mpsc::Sender<UiMessage>,
    controller_tx: mpsc::Sender<ControllerMessage>,
}

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

        let select_view = SelectView::<usize>::new().with_name("InstallDiskList");

        let select_dialog = Dialog::new()
            .title("Select installation disk")
            .content(ListView::new().child("", header).child("", select_view))
            .with_name("DiskSelectDialog");

        ui.cursive.add_layer(select_dialog);

        ui
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
                UiMessage::UpdateOutput(text) => {
                    let mut output = self.cursive.find_name::<TextView>("output").unwrap();
                    output.set_content(text);
                }
                UiMessage::ShowDiskList(disks) => {
                    let mut output = self
                        .cursive
                        .find_name::<SelectView<usize>>("InstallDiskList")
                        .unwrap();
                    output.clear();
                    for (i, disk) in disks.borrow().iter().enumerate() {
                        let s = format!("{disk:<20}{size}", disk = &disk.name, size = "10G");
                        output.add_item(&s, i);
                    }
                    //output.set_selection(0);
                    let tx = self.controller_tx.clone();
                    output.set_on_submit(|s, _i| {
                        s.pop_layer();
                        s.add_layer(Dialog::text("selected"))
                    });
                    output.set_on_select(move |_e, _l| {
                        tx.send(ControllerMessage::UpdatePartitions).unwrap();
                    })
                }
            }
        }
        self.cursive.step();
        true
    }
}

pub enum UiMessage {
    UpdateOutput(String),
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
