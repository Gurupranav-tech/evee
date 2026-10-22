use evee::core::{
    Application, Evee, GPUContext, Layer, info,
    input::KeyCode,
    window::{Window, WindowProps},
};
use evee::renderer::ImGuiLayer;
use evee::window::glfw_window::GLFWWindow;
use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

struct MyLayer;

impl Layer for MyLayer {
    fn on_attach(&mut self) {
        info!("My Layer attached");
    }

    fn on_detach(&mut self) {
        info!("My Layer detached");
    }

    fn on_update(&mut self, input_manager: &dyn evee::core::Input) {
        if input_manager.is_keypressed(KeyCode::Tab) {
            info!("Tab is being pressed");
        }
    }

    fn on_event(&mut self, _event: &mut evee::core::event::event::EventCategory) {}
}

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
        let my_layer = Box::new(MyLayer);
        vec![imgui_layer, my_layer]
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
