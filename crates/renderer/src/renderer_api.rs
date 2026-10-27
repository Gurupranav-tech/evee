use core::window::Window;
use std::{cell::RefCell, error::Error, rc::Rc};

use crate::{
    render_command::{self, RenderCommand},
    vertex_array::VertexArray,
};

pub struct Renderer {
    render_command: Box<dyn RenderCommand>,
}

impl Renderer {
    pub fn new(window: Rc<RefCell<Box<dyn Window>>>) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            render_command: render_command::create_render_command(window)?,
        })
    }

    pub fn clear(&self) {
        self.render_command.clear();
    }

    pub fn set_clear_color(&self, color: glam::Vec4) {
        self.render_command.set_clear_color(color);
    }

    pub fn begin(&self) {}

    pub fn end(&self) {}

    pub fn submit(&self, vertex_array: &dyn VertexArray) {
        vertex_array.bind();
        self.render_command.draw_indexed(vertex_array.count());
    }
}
