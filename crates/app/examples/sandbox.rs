#![allow(unused_variables, dead_code, unused_imports)]
use evee::renderer::ImGuiLayer;
use evee::window::glfw_window::GLFWWindow;
use evee::{
    core::{
        Application, Evee, GPUContext, Layer,
        window::{Window, WindowProps},
    },
    renderer::ImGuiAddOn,
};
use renderer::buffer::{self, IndexBuffer, VertexBuffer};
use renderer::glow::{self, HasContext};
use renderer::shader::Shader;
use renderer::vertex_array::{self, VertexArray};
use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

const VERTEX: &str = r#"
    #version 330 core

    layout(location = 0) in vec3 aPos;

    void main() {
        gl_Position = vec4(aPos, 1.0);
    }
"#;
const FRAGMENT: &str = r#"
    #version 330 core

    out vec4 color;

    void main() {
        color = vec4(1.0, 0.0, 0.0, 1.0);
    }
"#;

struct DummyLayer {
    shader: Shader,
    vbo: Rc<Box<dyn VertexBuffer>>,
    ibo: Rc<Box<dyn IndexBuffer>>,
    vao: Box<dyn VertexArray>,
    gl: Rc<renderer::glow::Context>,
}

impl Layer for DummyLayer {
    fn on_attach(&mut self) {
        self.vao.add_vertexbuffer(Rc::clone(&self.vbo));
        self.vao.add_indexbuffer(Rc::clone(&self.ibo));
    }

    fn on_detach(&mut self) {}

    fn on_update(&mut self, input_manager: &dyn core::Input) {
        self.shader.bind();
        unsafe {
            self.vao.bind();
            // self.vbo.bind();
            // self.ibo.bind();
            self.gl
                .draw_elements(glow::TRIANGLES, self.vao.count(), glow::UNSIGNED_INT, 0);
        }
    }

    fn on_event(&mut self, event: &mut core::event::event::EventCategory) {}
}

impl DummyLayer {
    fn new(window: Rc<RefCell<Box<dyn Window>>>) -> Self {
        let vertices: [f32; 12] = [-0.5, 0.0, 0.0, 0.5, 0.0, 0.0, 0.0, 0.5, 0.0, 0.0, -0.5, 0.0];
        let indices: [u32; 6] = [0, 1, 2, 0, 1, 3];

        let gl = window
            .borrow()
            .as_any()
            .downcast_ref::<GLFWWindow>()
            .expect("Need glfw window")
            .gl_context()
            .expect("Need opengl");

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
            gl,
            vao: vertex_array::create_vertexarray(Rc::clone(&window)).expect("Expected vao"),
        }
    }
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
        let imgui_layer = Box::new(ImGuiLayer::new(Rc::clone(&window)));
        let dummy_layer = Box::new(DummyLayer::new(Rc::clone(&window)));

        vec![imgui_layer, dummy_layer]
        // return vec![];
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
    let app = Box::new(MyApplication::new(1920, 1080, "Evee Sandbox".to_string()));
    // let app = Box::new(MyApplication::new(800, 600, "Evee Sandbox".to_string()));
    let mut evee = Evee::new(app)?.init();

    evee.run()?;

    Ok(())
}
