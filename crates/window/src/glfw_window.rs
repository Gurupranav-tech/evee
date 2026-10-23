use core::event::MouseEvent;
use core::event::event::{
    EventCategory,
    KeyEvent::{self, KeyReleaseEvent},
};
use core::graphics::GraphicsContext;
use core::input::{KeyCode, MouseButton};
use core::window::{Window, WindowProps};
use core::{GPUContext, Input, info, warn};
use glfw::{Glfw, GlfwReceiver, PWindow, WindowEvent};
use std::cell::RefCell;
use std::{collections::HashSet, error::Error, rc::Rc};

use crate::contexts::glfw::opengl_context::OpenglContext;

pub struct GLFWWindow {
    window_props: WindowProps,
    vsync: bool,
    event_callback: Option<Box<dyn FnMut(EventCategory)>>,

    // GLFW specific handles
    glfw: Glfw,
    window: Rc<RefCell<PWindow>>,
    events: GlfwReceiver<(f64, WindowEvent)>,
    key_pressed: HashSet<u32>,
    mousebtn_pressed: HashSet<u32>,

    // Graphics Context
    graphics_context: Option<Box<dyn GraphicsContext>>,
}

impl Input for GLFWWindow {
    fn is_keypressed(&self, _keycode: KeyCode) -> bool {
        self.key_pressed.contains(&(_keycode as u32))
    }

    fn get_mouse_position(&self) -> (f64, f64) {
        self.window.borrow().get_cursor_pos()
    }

    fn is_mousebtn_pressed(&self, _btn: MouseButton) -> bool {
        self.mousebtn_pressed.contains(&(_btn as u32))
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
        window.set_scroll_polling(true);
        window.set_char_polling(true);

        info!(
            "Created GLFW Window: {} {}x{} with context {:?}",
            props.title, props.width, props.height, props.context
        );

        let window = Rc::new(RefCell::new(window));

        let mut glfw_window = Box::new(Self {
            window_props: props,
            vsync: false,
            event_callback: None,
            glfw,
            events: event,
            key_pressed: HashSet::new(),
            mousebtn_pressed: HashSet::new(),
            window,
            graphics_context: None,
        });

        match &glfw_window.window_props.context {
            GPUContext::Opengl => {
                let mut context = OpenglContext::new(Rc::clone(&glfw_window.window));
                context.init();
                glfw_window.graphics_context = Some(Box::new(context));
            }

            GPUContext::None => warn!("No GPU Context was choosen"),
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

                    if let Some(ref graphics_context) = self.graphics_context {
                        graphics_context.viewport_update(*width as i32, *height as i32);
                    }
                }

                if let EventCategory::EventCategoryKeyboard(event) = &evee_event {
                    match event {
                        KeyEvent::KeyPressedEvent(keycode, _) => {
                            self.key_pressed.insert(*keycode);
                        }
                        KeyReleaseEvent(keycode) => {
                            self.key_pressed.remove(keycode);
                        }
                    }
                }

                if let EventCategory::EventCategoryMouse(event) = &evee_event {
                    match event {
                        MouseEvent::MouseButtonPressedEvent(btn) => {
                            self.mousebtn_pressed.insert(*btn);
                        }
                        MouseEvent::MouseButtonReleaseEvent(btn) => {
                            self.mousebtn_pressed.remove(btn);
                        }
                        _ => {}
                    }
                }

                if let Some(callback) = self.event_callback.as_mut() {
                    callback(evee_event);
                }
            }
        }

        let Some(grpahics_context) = &mut self.graphics_context else {
            return;
        };

        grpahics_context.swap_buffers();
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

    fn get_framebuffer_size(&self) -> (i32, i32) {
        self.window.borrow().get_framebuffer_size()
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
        let Some(grpahics_context) = &self.graphics_context else {
            return None;
        };

        if let GPUContext::Opengl = grpahics_context.context_type() {
            grpahics_context
                .as_any()
                .downcast_ref::<OpenglContext>()
                .expect("Not a opengl context")
                .gl()
        } else {
            None
        }
    }
}
