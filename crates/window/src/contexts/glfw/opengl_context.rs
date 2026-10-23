use core::{GPUContext, graphics::GraphicsContext};
use std::{any::Any, cell::RefCell, rc::Rc};

use glfw::{Context as GLFWContext, PWindow};
use glow::{Context as GlowContext, HasContext};

pub struct OpenglContext {
    window: Rc<RefCell<PWindow>>,
    gl: Option<Rc<GlowContext>>,

    vao: Option<glow::VertexArray>,
    vbo: Option<glow::Buffer>,
}

impl GraphicsContext for OpenglContext {
    fn init(&mut self) {
        self.window.borrow_mut().make_current();

        let gl = Rc::new(unsafe {
            GlowContext::from_loader_function(|symbol| {
                self.window.borrow_mut().get_proc_address(symbol) as *const _
            })
        });

        let (width, height) = self.window.borrow().get_size();

        unsafe {
            gl.enable(glow::DEPTH_TEST);
            gl.viewport(0, 0, width, height);

            self.vao = gl.create_vertex_array().ok();
            self.vbo = gl.create_buffer().ok();

            gl.bind_vertex_array(self.vao);
            gl.bind_buffer(glow::ARRAY_BUFFER, self.vbo);

            let vertices: [f32; 9] = [-0.5, -0.5, 0.0, 0.5, 0.5, 0.0, 0.5, -0.5, 0.0];

            let u8_vertices: &[u8] = std::slice::from_raw_parts(
                vertices.as_ptr() as *const u8,
                vertices.len() * std::mem::size_of::<f32>(),
            );
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, u8_vertices, glow::STATIC_DRAW);

            gl.enable_vertex_attrib_array(0);
            gl.vertex_attrib_pointer_f32(
                0,                                     // layout (location = 0)
                3,                                     // Size (3 coordinates per vertex: X, Y, Z)
                glow::FLOAT,                           // Data type
                false,                                 // Normalized
                3 * std::mem::size_of::<f32>() as i32, // Stride (byte size of single vertex context)
                0,                                     // Offset
            );
        }

        self.gl = Some(gl);
    }

    fn swap_buffers(&mut self) {
        self.window.borrow_mut().swap_buffers();
        let Some(ref mut gl) = self.gl else { return };

        unsafe {
            gl.clear_color(0.0, 0.0, 0.0, 1.0);
            gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);

            gl.bind_vertex_array(self.vao);
            gl.draw_arrays(
                glow::TRIANGLES, // Primitive type
                0,               // Starting array index
                3,               // Number of vertices to process
            );
        }
    }

    fn viewport_update(&self, width: i32, height: i32) {
        if let Some(ref gl) = self.gl {
            unsafe {
                gl.viewport(0, 0, width, height);
            }
        }
    }

    fn context_type(&self) -> GPUContext {
        GPUContext::Opengl
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl OpenglContext {
    pub fn new(window: Rc<RefCell<PWindow>>) -> Self {
        Self {
            window,
            gl: None,
            vao: None,
            vbo: None,
        }
    }

    pub fn gl(&self) -> Option<Rc<GlowContext>> {
        self.gl.clone()
    }
}
