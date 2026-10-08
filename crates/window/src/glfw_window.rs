use core::{
    GPUContext,
    event::event::EventCategory,
    info, warn,
    window::{Window, WindowProps},
};
use glfw::{Context, Glfw, GlfwReceiver, PWindow, WindowEvent};
use glow::HasContext;
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use std::{error::Error, rc::Rc};

pub struct GLFWWindow {
    window_props: WindowProps,
    vsync: bool,
    event_callback: Option<Box<dyn FnMut(EventCategory)>>,

    // GLFW specific handles
    glfw: Glfw,
    window: PWindow,
    events: GlfwReceiver<(f64, WindowEvent)>,

    // Graphics Context
    gl: Option<Rc<glow::Context>>,
}

impl HasWindowHandle for GLFWWindow {
    fn window_handle(
        &self,
    ) -> Result<raw_window_handle::WindowHandle<'_>, raw_window_handle::HandleError> {
        self.window.window_handle()
    }
}

impl HasDisplayHandle for GLFWWindow {
    fn display_handle(
        &self,
    ) -> Result<raw_window_handle::DisplayHandle<'_>, raw_window_handle::HandleError> {
        self.window.display_handle()
    }
}

impl Window for GLFWWindow {
    fn new(props: core::window::WindowProps) -> Result<Box<dyn Window>, Box<dyn Error>>
    where
        Self: Sized,
    {
        let mut glfw = glfw::init(glfw::fail_on_errors)?;

        let (mut window, event) = glfw
            .create_window(
                props.width,
                props.height,
                &props.title,
                glfw::WindowMode::Windowed,
            )
            .ok_or("Cannot create a GLFW Window")?;

        window.set_key_polling(true);
        window.set_size_polling(true);
        window.set_framebuffer_size_polling(true);
        window.set_close_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_mouse_button_polling(true);
        window.set_focus_polling(true);

        info!(
            "Created GLFW Window: {} {}x{} with context {:?}",
            props.title, props.width, props.height, props.context
        );

        let mut glfw_window = Box::new(Self {
            window_props: props,
            vsync: false,
            event_callback: None,
            glfw,
            window,
            events: event,
            gl: None,
        });

        match &glfw_window.window_props.context {
            core::GPUContext::Opengl => {
                glfw_window.window.make_current();

                let gl = Rc::new(unsafe {
                    glow::Context::from_loader_function(|symbol| {
                        glfw_window.window.get_proc_address(symbol) as *const _
                    })
                });

                unsafe { gl.enable(glow::DEPTH_TEST) };

                glfw_window.gl = Some(gl);
            }

            core::GPUContext::None => warn!("No GPU Context was choosen"),
        };

        Ok(glfw_window)
    }

    fn window_props(&self) -> &WindowProps {
        &self.window_props
    }

    fn window_type(&self) -> &str {
        "GLFW"
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn on_update(&mut self) {
        self.glfw.poll_events();

        for (_, event) in glfw::flush_messages(&self.events) {
            if let Some(evee_event) = self.translate(event) {
                if let EventCategory::EventCategoryApplication(
                    core::event::event::ApplicationEvent::WindowResizeEvent(width, height),
                ) = &evee_event
                {
                    self.window_props.width = *width;

                    self.window_props.height = *height;
                }

                if let Some(callback) = self.event_callback.as_mut() {
                    callback(evee_event);
                }
            }
        }

        match self.window_props.context {
            GPUContext::Opengl => {
                self.window.swap_buffers();
            }

            GPUContext::None => {}
        }
    }

    fn get_width(&self) -> u32 {
        self.window_props.width
    }

    fn get_height(&self) -> u32 {
        self.window_props.height
    }

    fn set_event_callback(&mut self, callback: Box<dyn FnMut(core::event::event::EventCategory)>) {
        self.event_callback = Some(callback);
    }

    fn set_vsync(&mut self, enabled: bool) {
        if enabled {
            self.glfw.set_swap_interval(glfw::SwapInterval::Adaptive);
        } else {
            self.glfw.set_swap_interval(glfw::SwapInterval::None);
        }

        self.vsync = enabled;
    }

    fn is_vsync(&self) -> bool {
        self.vsync
    }
}

impl GLFWWindow {
    fn translate(&self, event: glfw::WindowEvent) -> Option<EventCategory> {
        match event {
            glfw::WindowEvent::Close => Some(EventCategory::EventCategoryApplication(
                core::event::event::ApplicationEvent::WindowCloseEvent,
            )),

            glfw::WindowEvent::Size(width, height) => {
                Some(EventCategory::EventCategoryApplication(
                    core::event::event::ApplicationEvent::WindowResizeEvent(
                        width as u32,
                        height as u32,
                    ),
                ))
            }

            glfw::WindowEvent::Key(key, _scancode, action, _modifiers) => match action {
                glfw::Action::Press => Some(EventCategory::EventCategoryKeyboard(
                    core::event::event::KeyEvent::KeyPressedEvent(key as u32, 0),
                )),

                glfw::Action::Release => Some(EventCategory::EventCategoryKeyboard(
                    core::event::event::KeyEvent::KeyReleaseEvent(key as u32),
                )),

                glfw::Action::Repeat => Some(EventCategory::EventCategoryKeyboard(
                    core::event::event::KeyEvent::KeyPressedEvent(key as u32, 1),
                )),
            },

            glfw::WindowEvent::MouseButton(button, action, _mods) => match action {
                glfw::Action::Press => Some(EventCategory::EventCategoryMouse(
                    core::event::event::MouseEvent::MouseButtonPressedEvent(button as u32),
                )),

                glfw::Action::Release => Some(EventCategory::EventCategoryMouse(
                    core::event::event::MouseEvent::MouseButtonReleaseEvent(button as u32),
                )),

                glfw::Action::Repeat => Some(EventCategory::EventCategoryMouse(
                    core::event::event::MouseEvent::MouseButtonPressedEvent(button as u32),
                )),
            },

            glfw::WindowEvent::Scroll(off_x, off_y) => Some(EventCategory::EventCategoryMouse(
                core::event::event::MouseEvent::MouseScrollEvent(off_x as f32, off_y as f32),
            )),

            glfw::WindowEvent::CursorPos(x, y) => Some(EventCategory::EventCategoryMouse(
                core::event::event::MouseEvent::MouseMoveEvent(x as f32, y as f32),
            )),

            _ => None,
        }
    }

    pub fn gl_context(&self) -> Option<Rc<glow::Context>> {
        self.gl.clone()
    }
}
