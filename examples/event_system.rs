use std::error::Error;

use evee::{Application, event::event::ApplicationEvent::WindowResizeEvent, info};

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
}

impl MyApplication {
    fn new() -> Self {
        Self
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let app = Box::new(MyApplication::new());

    evee::run(app)?;

    Ok(())
}
