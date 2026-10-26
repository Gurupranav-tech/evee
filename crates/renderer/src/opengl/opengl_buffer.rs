use std::{error::Error, rc::Rc};

use crate::buffer::{BufferLayout, IndexBuffer, VertexBuffer};
use glow::{Context, HasContext};

pub struct OpenglVertexBuffer {
    gl: Rc<Context>,
    render_id: glow::Buffer,
    buffer_layout: Option<BufferLayout>,
}

impl VertexBuffer for OpenglVertexBuffer {
    fn bind(&self) {
        unsafe {
            self.gl
                .bind_buffer(glow::ARRAY_BUFFER, Some(self.render_id));
        }
    }

    fn unbind(&self) {
        unsafe {
            self.gl.bind_buffer(glow::ARRAY_BUFFER, None);
        }
    }

    fn set_layout(&mut self, buffer_layout: BufferLayout) {
        self.buffer_layout = Some(buffer_layout);
    }

    fn get_layout(&self) -> Option<&BufferLayout> {
        self.buffer_layout.as_ref()
    }
}

impl OpenglVertexBuffer {
    pub fn new(gl: Rc<Context>, vertices: &[f32]) -> Result<Self, Box<dyn Error>> {
        let render_id = unsafe { gl.create_buffer()? };

        unsafe {
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(render_id));
            let vertices = std::slice::from_raw_parts(
                vertices.as_ptr() as *const u8,
                vertices.len() * std::mem::size_of::<f32>(),
            );
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, vertices, glow::STATIC_DRAW);
        }

        Ok(Self {
            gl,
            render_id,
            buffer_layout: None,
        })
    }
}

impl Drop for OpenglVertexBuffer {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_buffer(self.render_id);
        }
    }
}

pub struct OpenglIndexBuffer {
    gl: Rc<Context>,
    render_id: glow::Buffer,
    count: i32,
}

impl IndexBuffer for OpenglIndexBuffer {
    fn bind(&self) {
        unsafe {
            self.gl
                .bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(self.render_id));
        }
    }

    fn unbind(&self) {
        unsafe {
            self.gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, None);
        }
    }

    fn count(&self) -> i32 {
        self.count
    }
}

impl OpenglIndexBuffer {
    pub fn new(gl: Rc<Context>, indices: &[u32]) -> Result<Self, Box<dyn Error>> {
        let render_id = unsafe { gl.create_buffer()? };
        let count = indices.len() as i32;

        unsafe {
            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(render_id));
            let vertices = std::slice::from_raw_parts(
                indices.as_ptr() as *const u8,
                indices.len() * std::mem::size_of::<u32>(),
            );
            gl.buffer_data_u8_slice(glow::ELEMENT_ARRAY_BUFFER, vertices, glow::STATIC_DRAW);
        }

        Ok(Self {
            gl,
            render_id,
            count,
        })
    }
}

impl Drop for OpenglIndexBuffer {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_buffer(self.render_id);
        }
    }
}
