use std::any::Any;

use crate::GPUContext;

pub trait GraphicsContext {
    fn init(&mut self);
    fn swap_buffers(&mut self);

    fn viewport_update(&self, width: i32, height: i32);

    fn as_any(&self) -> &dyn Any;

    fn context_type(&self) -> GPUContext {
        GPUContext::None
    }
}
