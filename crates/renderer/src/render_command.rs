use core::{GPUContext, window::Window};
use std::{cell::RefCell, error::Error, rc::Rc};

use window::glfw_window::GLFWWindow;

use crate::opengl::opengl_render_command::OpenglRenderCommand;

pub trait RenderCommand {
    fn clear(&self);
    fn set_clear_color(&self, color: glam::Vec4);

    fn draw_indexed(&self, count: i32);
}

pub fn create_render_command(
    window: Rc<RefCell<Box<dyn Window>>>,
) -> Result<Box<dyn RenderCommand>, Box<dyn Error>> {
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

        return Ok(Box::new(OpenglRenderCommand::new(gl)?));
    }

    Err("Unknown graphics api".into())
}
