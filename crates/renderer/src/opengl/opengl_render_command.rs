use glow::HasContext;

use crate::render_command::RenderCommand;
use std::{error::Error, rc::Rc};

pub struct OpenglRenderCommand {
    gl: Rc<glow::Context>,
}

impl RenderCommand for OpenglRenderCommand {
    fn clear(&self) {
        unsafe {
            self.gl
                .clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
        }
    }

    fn set_clear_color(&self, color: glam::Vec4) {
        unsafe {
            self.gl.clear_color(color.x, color.y, color.z, color.w);
        }
    }

    fn draw_indexed(&self, count: i32) {
        unsafe {
            self.gl
                .draw_elements(glow::TRIANGLES, count, glow::UNSIGNED_INT, 0);
        }
    }
}

impl OpenglRenderCommand {
    pub fn new(gl: Rc<glow::Context>) -> Result<Self, Box<dyn Error>> {
        Ok(Self { gl })
    }
}
