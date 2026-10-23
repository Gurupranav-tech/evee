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
use renderer::buffer::{IndexBuffer, VertexBuffer, create_indexbuffer, create_vertexbuffer};
use renderer::glow::{self, HasContext};
use renderer::shader::Shader;
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
    vbo: Box<dyn VertexBuffer>,
    ibo: Box<dyn IndexBuffer>,
    vao: glow::VertexArray,
    gl: Rc<renderer::glow::Context>,
}

impl Layer for DummyLayer {
    fn on_attach(&mut self) {
        unsafe {
            self.gl.bind_vertex_array(Some(self.vao));
            self.gl.enable_vertex_attrib_array(0);
            self.gl.vertex_attrib_pointer_f32(
                0,
                3,
                glow::FLOAT,
                false,
                std::mem::size_of::<f32>() as i32,
                0,
            );
        }
    }

    fn on_detach(&mut self) {}

    fn on_update(&mut self, input_manager: &dyn core::Input) {
        self.shader.bind();
        unsafe {
            self.gl.bind_vertex_array(Some(self.vao));
            self.vbo.bind();
            self.ibo.bind();
            self.gl
                .draw_elements(glow::TRIANGLES, self.ibo.count(), glow::UNSIGNED_INT, 0);
        }
    }

    fn on_event(&mut self, event: &mut core::event::event::EventCategory) {}
}

impl DummyLayer {
    fn new(window: Rc<RefCell<Box<dyn Window>>>) -> Self {
        let vertices: [f32; 9] = [-0.5, 0.0, 0.0, 0.5, 0.0, 0.0, 0.0, 0.5, 0.0];
        let indices: [u32; 3] = [0, 1, 2];

        let gl = window
            .borrow()
            .as_any()
            .downcast_ref::<GLFWWindow>()
            .expect("Need glfw window")
            .gl_context()
            .expect("Need opengl");

        let vertex_array = unsafe { gl.create_vertex_array().expect("Need vertex array") };
        unsafe {
            gl.bind_vertex_array(Some(vertex_array));
        }

        Self {
            shader: Shader::new(Rc::clone(&window), VERTEX, FRAGMENT).expect("Err"),
            vbo: create_vertexbuffer(Rc::clone(&window), &vertices[..]).expect("Expected a vbo"),
            ibo: create_indexbuffer(Rc::clone(&window), &indices[..]).expect("Expected ebo"),
            gl,
            vao: vertex_array,
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
