use core::{GPUContext, window::Window};
use std::{cell::RefCell, error::Error, rc::Rc};

use window::glfw_window::GLFWWindow;

use crate::opengl::opengl_buffer::{OpenglIndexBuffer, OpenglVertexBuffer};

pub trait VertexBuffer {
    fn bind(&self);
    fn unbind(&self);
}

pub fn create_vertexbuffer(
    window: Rc<RefCell<Box<dyn Window>>>,
    vertices: &[f32],
) -> Result<Box<dyn VertexBuffer>, Box<dyn Error>> {
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

        return Ok(Box::new(OpenglVertexBuffer::new(gl, vertices)?));
    }

    Err("Unknown graphics api".into())
}
pub trait IndexBuffer {
    fn bind(&self);
    fn unbind(&self);

    fn count(&self) -> i32;
}

pub fn create_indexbuffer(
    window: Rc<RefCell<Box<dyn Window>>>,
    indices: &[u32],
) -> Result<Box<dyn IndexBuffer>, Box<dyn Error>> {
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

        return Ok(Box::new(OpenglIndexBuffer::new(gl, indices)?));
    }

    Err("Unknown graphics api".into())
}
