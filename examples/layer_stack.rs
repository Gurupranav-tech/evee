use evee::core::{
    Application, Evee, GPUContext, Layer, info,
    window::{Window, WindowProps},
};
use evee::window::glfw_window::GLFWWindow;
use std::error::Error;

struct ExampleLayer;

impl Layer for ExampleLayer {
    fn on_attach(&mut self) {
        info!("Example Layer Attached");
    }

    fn on_detach(&mut self) {
        info!("Example Layer Deattached");
    }

    fn on_update(&mut self) {}

    fn on_event(&mut self, event: &mut evee::core::event::event::EventCategory) {
        info!("Example Layer received an event {:?}", event);
    }
}

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
            context: GPUContext::None,
        })
    }

    fn create_layers(&self) -> Vec<Box<dyn Layer>> {
        vec![Box::new(ExampleLayer {})]
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
