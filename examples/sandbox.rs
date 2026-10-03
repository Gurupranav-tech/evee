use evee::core::{
    Application, Evee, GPUContext, Layer, info,
    window::{Window, WindowProps},
};
use evee::window::glfw_window::GLFWWindow;
use std::error::Error;

struct MyApplication {
    width: u32,
    height: u32,
    title: String,
}

impl Application for MyApplication {
    fn create_window(&self) -> Result<Box<dyn Window>, Box<dyn Error>> {
        GLFWWindow::new(WindowProps {
            width: self.width,
            height: self.height,
            title: self.title.clone(),
            context: GPUContext::Opengl,
        })
    }
}

impl MyApplication {
    fn new(width: u32, height: u32, title: String) -> Self {
        Self {
            width,
            height,
            title,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let app = Box::new(MyApplication::new(800, 600, "Evee Sandbox".to_string()));
    let mut evee = Evee::new(app)?.init();

    evee.run()?;

    Ok(())
}
