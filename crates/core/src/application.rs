use std::error::Error;

use crate::{logger::init_logger, window::Window};

pub trait Application {
    fn run(&self);
    fn create_window(&self) -> Result<Box<dyn Window>, Box<dyn Error>>;
}

pub struct Evee {
    app: Box<dyn Application>,
    running: bool,
    window: Box<dyn Window>,
}

impl Evee {
    pub fn new(app: Box<dyn Application>) -> Result<Self, Box<dyn Error>> {
        init_logger();

        let window = app.create_window()?;

        Ok(Self {
            app,
            window,
            running: true,
        })
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        while self.running {
            self.window.on_update();

            self.app.run();
        }

        Ok(())
    }
}
