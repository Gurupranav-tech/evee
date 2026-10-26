use crate::buffer::ShaderDataType;

pub fn shadertype_to_opengltype(shader_type: &ShaderDataType) -> u32 {
    match shader_type {
        ShaderDataType::Bool => glow::BOOL,
        ShaderDataType::Float => glow::FLOAT,
        ShaderDataType::Float2 => glow::FLOAT,
        ShaderDataType::Float3 => glow::FLOAT,
        ShaderDataType::Float4 => glow::FLOAT,
        ShaderDataType::Int => glow::INT,
        ShaderDataType::Int2 => glow::INT,
        ShaderDataType::Int3 => glow::INT,
        ShaderDataType::Int4 => glow::INT,
        ShaderDataType::Mat3 => glow::FLOAT,
        ShaderDataType::Mat4 => glow::FLOAT,
        ShaderDataType::None => {
            core::error!("Invalid Shader type");
            panic!("Invalid Shader type");
        }
    }
}
