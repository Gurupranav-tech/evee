use core::window::{Window, WindowType};
use glow::{Context, HasContext};
use std::{cell::RefCell, error::Error, rc::Rc};
use window::glfw_window::GLFWWindow;

pub struct Shader {
    shader_program: glow::Program,
    gl: Rc<Context>,
}

impl Shader {
    pub fn new(
        window: Rc<RefCell<Box<dyn Window>>>,
        vertex: &str,
        fragment: &str,
    ) -> Result<Self, Box<dyn Error>> {
        let window = window.borrow();
        if !matches!(window.window_type(), WindowType::GLFW) {
            return Err("This window is not supported for opengl".into());
        }
        let gl = window
            .as_any()
            .downcast_ref::<GLFWWindow>()
            .ok_or("This window is not supported for opengl and this window is not glfw window")?
            .gl_context()
            .ok_or("Only Supported for Opengl")?;

        let shader_program = unsafe { gl.create_program()? };
        let vertex_shader = unsafe { gl.create_shader(glow::VERTEX_SHADER)? };
        let fragment_shader = unsafe { gl.create_shader(glow::FRAGMENT_SHADER)? };

        unsafe {
            gl.shader_source(vertex_shader, vertex);
            gl.compile_shader(vertex_shader);

            if !gl.get_shader_compile_status(vertex_shader) {
                let log = gl.get_shader_info_log(vertex_shader);
                core::error!("Error compiling vertex shader: {log}");
                return Err(log.into());
            }
        }

        unsafe {
            gl.shader_source(fragment_shader, fragment);
            gl.compile_shader(fragment_shader);

            if !gl.get_shader_compile_status(fragment_shader) {
                let log = gl.get_shader_info_log(fragment_shader);
                core::error!("Error compiling fragment shader: {log}");
                return Err(log.into());
            }
        }

        unsafe {
            gl.attach_shader(shader_program, vertex_shader);
            gl.attach_shader(shader_program, fragment_shader);
            gl.link_program(shader_program);

            if !gl.get_program_link_status(shader_program) {
                let log = gl.get_program_info_log(shader_program);
                core::error!("Error compiling fragment shader: {log}");
                return Err(log.into());
            }

            gl.detach_shader(shader_program, vertex_shader);
            gl.detach_shader(shader_program, fragment_shader);
        }

        Ok(Self { gl, shader_program })
    }

    pub fn bind(&self) {
        unsafe {
            self.gl.use_program(Some(self.shader_program));
        }
    }

    pub fn unbind(&self) {
        unsafe {
            self.gl.use_program(None);
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_program(self.shader_program);
        }
    }
}
