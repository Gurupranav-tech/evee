use std::error::Error;

use evee::core::{
    Application, Evee, event::event::ApplicationEvent::WindowResizeEvent, info, window::Window,
};

struct MyApplication;

impl Application for MyApplication {
    fn run(&self) {
        let event = WindowResizeEvent(400, 600);
        info!(
            "Event is {:?} and the string form is {}",
            event,
            event.to_string()
        );
    }

    fn create_window(&self) -> Result<Box<dyn Window>, Box<dyn std::error::Error>> {
        todo!()
    }
}

impl MyApplication {
    fn new() -> Self {
        Self
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let app = Box::new(MyApplication::new());
    let mut evee = Evee::new(app)?;

    evee.run()?;

    Ok(())
}
