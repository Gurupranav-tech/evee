use evee::core::{
    Application, Evee, GPUContext, Layer,
    window::{Window, WindowProps},
};
use evee::renderer::ImGuiLayer;
use evee::window::glfw_window::GLFWWindow;
use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

struct MyApplication {
    width: u32,
    height: u32,
    title: String,
    window: Option<Rc<RefCell<Box<dyn Window>>>>,
}

impl Application for MyApplication {
    fn create_window(&mut self) -> Result<Rc<RefCell<Box<dyn Window>>>, Box<dyn Error>> {
        let window = Rc::new(RefCell::new(GLFWWindow::new(WindowProps {
            width: self.width,
            height: self.height,
            title: self.title.clone(),
            context: GPUContext::Opengl,
        })?));
        let new_window = Rc::clone(&window);

        self.window = Some(window);
        Ok(new_window)
    }

    fn create_layers(&self, window: Rc<RefCell<Box<dyn Window>>>) -> Vec<Box<dyn Layer>> {
        let imgui_layer = Box::new(ImGuiLayer::new(window));
        vec![imgui_layer]
    }
}

impl MyApplication {
    fn new(width: u32, height: u32, title: String) -> Self {
        Self {
            width,
            height,
            title,
            window: None,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let app = Box::new(MyApplication::new(800, 600, "Evee Sandbox".to_string()));
    let mut evee = Evee::new(app)?.init();

    evee.run()?;

    Ok(())
}
