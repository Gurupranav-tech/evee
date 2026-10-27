#![allow(unused_variables, dead_code, unused_imports)]
use core::error;
use evee::application::Evee;
use evee::renderer::ImGuiLayer;
use evee::window::glfw_window::GLFWWindow;
use evee::{
    core::{
        Application, GPUContext, Layer,
        window::{Window, WindowProps},
    },
    renderer::ImGuiAddOn,
};
use renderer::buffer::{self, IndexBuffer, VertexBuffer};
use renderer::renderer_api::Renderer;
use renderer::shader::Shader;
use renderer::vertex_array::{self, VertexArray};
use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

const VERTEX: &str = r#"
    #version 330 core

    layout(location = 0) in vec3 aPos;
    out vec4 out_color;

    void main() {
        gl_Position = vec4(aPos, 1.0);
        out_color = vec4(aPos + 0.5, 1.0);
    }
"#;
const FRAGMENT: &str = r#"
    #version 330 core

    out vec4 color;
    in vec4 out_color;

    void main() {
        // color = vec4(1.0, 0.0, 0.0, 1.0);
        color = out_color;
    }
"#;

struct DummyLayer {
    shader: Shader,
    vbo: Rc<Box<dyn VertexBuffer>>,
    ibo: Rc<Box<dyn IndexBuffer>>,
    vao: Box<dyn VertexArray>,
    renderer: Rc<Renderer>,
}

impl Layer for DummyLayer {
    fn on_attach(&mut self) {
        self.vao.add_vertexbuffer(Rc::clone(&self.vbo));
        self.vao.add_indexbuffer(Rc::clone(&self.ibo));
    }

    fn on_detach(&mut self) {}

    fn on_update(&mut self, input_manager: &dyn core::Input) {
        self.shader.bind();
        self.renderer.begin();
        self.renderer.submit(self.vao.as_ref());
        self.renderer.end();
    }

    fn on_event(&mut self, event: &mut core::event::event::EventCategory) {}
}

impl DummyLayer {
    fn new(window: Rc<RefCell<Box<dyn Window>>>, renderer: Rc<Renderer>) -> Self {
        let vertices: [f32; 12] = [-0.5, 0.0, 0.0, 0.5, 0.0, 0.0, 0.0, 0.5, 0.0, 0.0, -0.5, 0.0];
        let indices: [u32; 6] = [0, 1, 2, 0, 1, 3];

        let buffer_layout = buffer::BufferLayout::new(vec![buffer::BufferElement::new(
            "Position".into(),
            buffer::ShaderDataType::Float3,
            false,
        )]);
        core::info!("Buffer layout: {:?}", buffer_layout);

        let mut vbo =
            buffer::create_vertexbuffer(Rc::clone(&window), &vertices[..]).expect("Expected a vbo");
        vbo.set_layout(buffer_layout);

        Self {
            shader: Shader::new(Rc::clone(&window), VERTEX, FRAGMENT).expect("Err"),
            vbo: Rc::new(vbo),
            ibo: Rc::new(
                buffer::create_indexbuffer(Rc::clone(&window), &indices[..]).expect("Expected ebo"),
            ),
            vao: vertex_array::create_vertexarray(Rc::clone(&window)).expect("Expected vao"),
            renderer,
        }
    }
}

struct UiAddon;

impl ImGuiAddOn for UiAddon {
    fn render(&mut self, ui: &mut imgui::Ui) {
        ui.show_demo_window(&mut true);
    }
}

struct MyApplication {
    width: u32,
    height: u32,
    title: String,
    window: Option<Rc<RefCell<Box<dyn Window>>>>,
    renderer: Option<Rc<Renderer>>,
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

        self.renderer = Some(Rc::new(Renderer::new(Rc::clone(&window))?));
        self.window = Some(window);
        Ok(new_window)
    }

    fn create_overlays(&self, window: Rc<RefCell<Box<dyn Window>>>) -> Vec<Box<dyn Layer>> {
        if let None = self.renderer {
            error!("Should have created a renderer by now");
            panic!("Should have created a renderer by now");
        }
        let dummy_layer = Box::new(DummyLayer::new(
            Rc::clone(&window),
            Rc::clone(&self.renderer.as_ref().unwrap()),
        ));

        vec![dummy_layer]
    }

    fn create_layers(&self, window: Rc<RefCell<Box<dyn Window>>>) -> Vec<Box<dyn Layer>> {
        let mut imgui_layer = Box::new(ImGuiLayer::new(Rc::clone(&window)));
        imgui_layer.add_ui_callback(Box::new(UiAddon));
        vec![imgui_layer]
    }

    fn clear(&self) {
        if let Some(renderer) = &self.renderer {
            renderer.clear();
        }
    }

    fn set_clear_color(&self, color: glam::Vec4) {
        if let Some(renderer) = &self.renderer {
            renderer.set_clear_color(color);
        }
    }
}

impl MyApplication {
    fn new(width: u32, height: u32, title: String) -> Self {
        Self {
            width,
            height,
            title,
            window: None,
            renderer: None,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let app = Box::new(MyApplication::new(1920, 1080, "Evee Sandbox".to_string()));
    let mut evee = Evee::new(app)?.init();

    evee.run()?;

    Ok(())
}
