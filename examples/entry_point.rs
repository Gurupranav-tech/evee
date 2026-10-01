use std::error::Error;

use evee::{Application, info};

struct MyApplication;

impl Application for MyApplication {
    fn run(&self) {
        info!("Evee Engine");
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
