use std::{error::Error, rc::Rc};

use glow::HasContext;

use crate::{
    buffer::{IndexBuffer, VertexBuffer},
    opengl::utils,
    vertex_array::VertexArray,
};

pub struct OpenglVertexArray {
    gl: Rc<glow::Context>,
    vertex_buffers: Vec<Rc<Box<dyn VertexBuffer>>>,
    index_buffers: Vec<Rc<Box<dyn IndexBuffer>>>,
    render_id: glow::VertexArray,
}

impl OpenglVertexArray {
    pub fn new(gl: Rc<glow::Context>) -> Result<Self, Box<dyn Error>> {
        let render_id = unsafe { gl.create_vertex_array()? };

        Ok(Self {
            gl,
            vertex_buffers: vec![],
            index_buffers: vec![],
            render_id,
        })
    }
}

impl VertexArray for OpenglVertexArray {
    fn bind(&self) {
        unsafe {
            self.gl.bind_vertex_array(Some(self.render_id));
        }
    }

    fn unbind(&self) {
        unsafe {
            self.gl.bind_vertex_array(None);
        }
    }

    fn count(&self) -> i32 {
        if self.index_buffers.len() == 0 {
            core::error!("No index buffer bound to the vertex array");
            return 0;
        }
        self.index_buffers[0].count()
    }

    fn add_vertexbuffer(&mut self, vertex_buffer: Rc<Box<dyn VertexBuffer>>) {
        self.bind();
        vertex_buffer.bind();
        let Some(layout) = vertex_buffer.get_layout() else {
            core::error!("Vertex buffer has no layout");
            return;
        };
        for (idx, element) in layout.get_elements().iter().enumerate() {
            unsafe {
                self.gl.enable_vertex_attrib_array(idx as u32);
                self.gl.vertex_attrib_pointer_f32(
                    idx as u32,
                    element.element_count(),
                    utils::shadertype_to_opengltype(element.field_type()),
                    element.normalized(),
                    layout.get_stride(),
                    element.get_offset(),
                );
            }
        }

        self.vertex_buffers.push(vertex_buffer);
    }

    fn add_indexbuffer(&mut self, index_buffer: Rc<Box<dyn IndexBuffer>>) {
        self.bind();
        index_buffer.bind();
        self.index_buffers.push(index_buffer);
    }
}

impl Drop for OpenglVertexArray {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_vertex_array(self.render_id);
        }
    }
}
