use crate::{
    event::event::{ApplicationEvent, EventCategory},
    logger::init_logger,
    window::Window,
};
use std::{error::Error, sync::mpsc};

pub trait Application {
    fn run(&self);
    fn create_window(&self) -> Result<Box<dyn Window>, Box<dyn Error>>;
}

pub struct Evee {
    app: Box<dyn Application>,
    running: bool,
    window: Box<dyn Window>,
    event_sender: mpsc::Sender<EventCategory>,
    event_receiver: mpsc::Receiver<EventCategory>,
}

impl Evee {
    pub fn new(app: Box<dyn Application>) -> Result<Self, Box<dyn Error>> {
        init_logger();

        let window = app.create_window()?;

        let (event_sender, event_receiver) = mpsc::channel();

        Ok(Self {
            app,
            window,
            running: true,
            event_receiver,
            event_sender,
        })
    }

    pub fn window_event(&mut self) {
        while let Ok(event_category) = self.event_receiver.try_recv() {
            tracing::info!("Received event: {:?}", event_category);
            match event_category {
                EventCategory::EventCategoryApplication(ApplicationEvent::WindowCloseEvent) => {
                    self.running = false
                }
                _ => {}
            }
        }
    }

    pub fn init(mut self) -> Self {
        let sender = self.event_sender.clone();

        (&mut self)
            .window
            .set_event_callback(Box::new(move |event_category: EventCategory| {
                sender.send(event_category).expect("Event System broken");
            }));
        self
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        while self.running {
            self.window.on_update();
            self.window_event();

            self.app.run();
        }

        Ok(())
    }
}
