use core::{GPUContext, window::Window};
use std::{cell::RefCell, error::Error, rc::Rc};

use window::glfw_window::GLFWWindow;

use crate::{
    buffer::{IndexBuffer, VertexBuffer},
    opengl::opengl_vertexarray::OpenglVertexArray,
};

pub trait VertexArray {
    fn bind(&self);
    fn unbind(&self);

    fn add_vertexbuffer(&mut self, vertex_buffer: Rc<Box<dyn VertexBuffer>>);
    fn add_indexbuffer(&mut self, index_buffer: Rc<Box<dyn IndexBuffer>>);
    fn count(&self) -> i32;
}

pub fn create_vertexarray(
    window: Rc<RefCell<Box<dyn Window>>>,
) -> Result<Box<dyn VertexArray>, Box<dyn Error>> {
    let window = window.try_borrow()?;

    if matches!(window.window_props().context, GPUContext::None) {
        return Err("Need graphics context".into());
    }

    if matches!(window.window_props().context, GPUContext::Opengl) {
        let gl = window
            .as_any()
            .downcast_ref::<GLFWWindow>()
            .ok_or("Only GLFW Window supports opengl for now")?
            .gl_context()
            .ok_or("Need opengl context")?;

        return Ok(Box::new(OpenglVertexArray::new(gl)?));
    }

    Err("Unknown graphics api".into())
}
