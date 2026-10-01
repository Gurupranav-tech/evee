use std::error::Error;

use evee::core::{Application, Evee, info, window::Window};

struct MyApplication;

impl Application for MyApplication {
    fn run(&self) {
        info!("Evee Engine");
    }

    fn create_window(&self) -> Result<Box<dyn Window>, Box<dyn std::error::Error>> {
        todo!();
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
