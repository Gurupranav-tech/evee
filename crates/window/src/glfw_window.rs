use core::{
    event::event::EventCategory,
    info,
    window::{Window, WindowProps},
};
use std::error::Error;

use glfw::{Context, Glfw, GlfwReceiver, PWindow, WindowEvent};

pub struct GLFWWindow {
    window_props: WindowProps,
    vsync: bool,
    event_callback: Option<Box<dyn FnMut(EventCategory)>>,

    // GLFW specific handles
    glfw: Glfw,
    window: PWindow,
    events: GlfwReceiver<(f64, WindowEvent)>,
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
        window.make_current();
        window.set_key_polling(true);
        window.set_size_polling(true);
        window.set_framebuffer_size_polling(true);
        window.set_close_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_mouse_button_polling(true);
        window.set_focus_polling(true);

        info!(
            "Created GLFW Window: {} {}x{}",
            props.title, props.width, props.height
        );

        Ok(Box::new(Self {
            window_props: WindowProps {
                width: props.width,
                height: props.height,
                title: props.title,
            },
            vsync: false,
            event_callback: None,
            glfw,
            window,
            events: event,
        }))
    }

    fn on_update(&mut self) {
        self.glfw.poll_events();

        for (_, event) in glfw::flush_messages(&self.events) {
            if let Some(evee_event) = self.translate(event) {
                if let Some(callback) = self.event_callback.as_mut() {
                    callback(evee_event);
                }
            }
        }

        self.window.swap_buffers();
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
            _ => None,
        }
    }
}
