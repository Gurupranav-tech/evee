use glfw::{Context, PWindow};

pub struct OpenglContext;

impl OpenglContext {
    pub fn create_context(window: &mut PWindow) {
        window.make_current();

        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        unsafe {
            gl::Enable(gl::DEPTH_TEST);
        }
    }

    pub fn swap_buffers(window: &mut PWindow) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        window.swap_buffers();
    }
}
