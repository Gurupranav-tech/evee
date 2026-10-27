use core::{GPUContext, graphics::GraphicsContext};
use std::{any::Any, cell::RefCell, rc::Rc};

use glfw::{Context as GLFWContext, PWindow};
use glow::{Context as GlowContext, HasContext};

pub struct OpenglContext {
    window: Rc<RefCell<PWindow>>,
    gl: Option<Rc<GlowContext>>,
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
        }

        self.gl = Some(gl);
    }

    fn swap_buffers(&mut self) {
        self.window.borrow_mut().swap_buffers();
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
        Self { window, gl: None }
    }

    pub fn gl(&self) -> Option<Rc<GlowContext>> {
        self.gl.clone()
    }
}
