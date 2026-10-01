use crate::{Application, logger::init_logger};
use std::error::Error;

pub fn run(app: Box<dyn Application>) -> Result<(), Box<dyn Error>> {
    init_logger();

    app.run();

    Ok(())
}
