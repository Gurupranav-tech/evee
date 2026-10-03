use crate::{
    event::event::{ApplicationEvent, EventCategory},
    layer_stack::{Layer, LayerStack},
    logger::init_logger,
    window::Window,
};
use std::{error::Error, sync::mpsc};

pub trait Application {
    fn create_window(&self) -> Result<Box<dyn Window>, Box<dyn Error>>;

    fn create_layers(&self) -> Vec<Box<dyn Layer>> {
        vec![]
    }

    fn create_overlays(&self) -> Vec<Box<dyn Layer>> {
        vec![]
    }
}

pub struct Evee {
    app: Box<dyn Application>,
    running: bool,

    // Window events
    window: Box<dyn Window>,
    event_sender: mpsc::Sender<EventCategory>,
    event_receiver: mpsc::Receiver<EventCategory>,

    // Layer Stack
    layer_stack: LayerStack,
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
            layer_stack: LayerStack::new(),
        })
    }

    fn window_event(&mut self) {
        while let Ok(mut event_category) = self.event_receiver.try_recv() {
            match event_category {
                EventCategory::EventCategoryApplication(ApplicationEvent::WindowCloseEvent) => {
                    self.running = false
                }
                _ => {
                    for layer in self.layer_stack.layers.iter_mut().rev() {
                        layer.on_event(&mut event_category);
                    }
                }
            }
        }
    }

    pub fn init(mut self) -> Self {
        let sender = self.event_sender.clone();

        for layer in self.app.create_layers() {
            self.layer_stack.push_layer(layer);
        }

        for overlay in self.app.create_overlays() {
            self.layer_stack.push_overlay(overlay);
        }

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

            for layer in &mut self.layer_stack.layers {
                layer.on_update();
            }
        }

        Ok(())
    }
}
