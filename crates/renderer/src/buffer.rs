use core::{GPUContext, window::Window};
use std::{cell::RefCell, error::Error, rc::Rc};

use window::glfw_window::GLFWWindow;

use crate::opengl::opengl_buffer::{OpenglIndexBuffer, OpenglVertexBuffer};

#[derive(Debug)]
pub enum ShaderDataType {
    None,
    Float,
    Float2,
    Float3,
    Float4,
    Mat3,
    Mat4,
    Int,
    Int2,
    Int3,
    Int4,
    Bool,
}

impl ShaderDataType {
    pub fn element_count(&self) -> i32 {
        match self {
            Self::Float => 1,
            Self::Float2 => 1 * 2,
            Self::Float3 => 1 * 3,
            Self::Float4 => 1 * 4,
            Self::Mat3 => 1 * 3 * 3,
            Self::Mat4 => 1 * 4 * 4,
            Self::Int => 1,
            Self::Int2 => 1 * 2,
            Self::Int3 => 1 * 3,
            Self::Int4 => 1 * 4,
            Self::Bool => 1,
            _ => 0,
        }
    }

    pub fn size(&self) -> i32 {
        let float = std::mem::size_of::<f32>() as i32;
        let uint = std::mem::size_of::<u32>() as i32;

        match self {
            Self::Float => 4,
            Self::Float2 => float * 2,
            Self::Float3 => float * 3,
            Self::Float4 => float * 4,
            Self::Mat3 => float * 3 * 3,
            Self::Mat4 => float * 4 * 4,
            Self::Int => uint,
            Self::Int2 => uint * 2,
            Self::Int3 => uint * 3,
            Self::Int4 => uint * 4,
            Self::Bool => std::mem::size_of::<bool>() as i32,
            _ => 0,
        }
    }
}

#[derive(Debug)]
pub struct BufferElement {
    pub name: String,
    offset: i32,
    size: i32,
    field_type: ShaderDataType,
    normalized: bool,
}

impl BufferElement {
    pub fn new(name: String, field_type: ShaderDataType, normalized: bool) -> Self {
        Self {
            name,
            size: field_type.size(),
            field_type,
            offset: 0,
            normalized,
        }
    }

    pub fn element_count(&self) -> i32 {
        self.field_type.element_count()
    }

    pub fn normalized(&self) -> bool {
        self.normalized
    }

    pub fn field_type(&self) -> &ShaderDataType {
        &self.field_type
    }

    pub fn get_offset(&self) -> i32 {
        self.offset
    }
}

#[derive(Debug)]
pub struct BufferLayout {
    elements: Vec<BufferElement>,
    stride: i32,
}

impl BufferLayout {
    pub fn get_elements(&self) -> &Vec<BufferElement> {
        &self.elements
    }

    pub fn new(mut elements: Vec<BufferElement>) -> Self {
        let mut offset: i32 = 0;
        let mut stride: i32 = 0;

        for element in &mut elements {
            element.offset = offset;
            offset += element.size;
            stride += element.size;
        }

        Self { elements, stride }
    }

    pub fn get_stride(&self) -> i32 {
        self.stride
    }
}

pub trait VertexBuffer {
    fn bind(&self);
    fn unbind(&self);

    fn set_layout(&mut self, buffer_layout: BufferLayout);
    fn get_layout(&self) -> Option<&BufferLayout>;
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
