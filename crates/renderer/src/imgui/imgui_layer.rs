use core::{
    GPUContext::{self},
    Input, Layer, error,
    event::{
        MouseEvent,
        event::{EventCategory, KeyEvent},
    },
    window::{Window, WindowType},
};
use std::{cell::RefCell, rc::Rc, time::Instant};

use imgui::Context as ImGuiContext;
use imgui::Key;
use imgui_glow_renderer::{Renderer, SimpleTextureMap};
use window::glfw_window::GLFWWindow;

use crate::ImGuiAddOn;

pub struct ImGuiLayer {
    window: Rc<RefCell<Box<dyn Window>>>,
    imgui: Option<ImGuiContext>,
    glow_renderer: Option<Renderer>,
    texture_map: SimpleTextureMap,
    last_frame: Instant,
    gl: Option<Rc<glow::Context>>,

    // callbacks
    ui_callbacks: Vec<Box<dyn ImGuiAddOn>>,
}

impl ImGuiLayer {
    pub fn new(window: Rc<RefCell<Box<dyn Window>>>) -> ImGuiLayer {
        let imgui_layer = Self {
            window,
            glow_renderer: None,
            imgui: None,
            texture_map: SimpleTextureMap::default(),
            last_frame: Instant::now(),
            gl: None,
            ui_callbacks: Vec::new(),
        };

        imgui_layer
    }
}

impl Layer for ImGuiLayer {
    fn on_attach(&mut self) {
        let mut imgui = ImGuiContext::create();

        let io = imgui.io_mut();
        io.config_flags.insert(imgui::ConfigFlags::DOCKING_ENABLE);
        io.config_flags.insert(imgui::ConfigFlags::VIEWPORTS_ENABLE);
        io.config_flags
            .insert(imgui::ConfigFlags::NAV_ENABLE_GAMEPAD);
        io.config_flags
            .insert(imgui::ConfigFlags::NAV_ENABLE_KEYBOARD);
        io.config_flags.insert(imgui::ConfigFlags::VIEWPORTS_ENABLE);

        io.display_size = [
            self.window.borrow().window_props().width as f32,
            self.window.borrow().window_props().height as f32,
        ];
        imgui.style_mut().use_dark_colors();

        match self.window.borrow().window_props().context {
            GPUContext::Opengl => {
                if !matches!(self.window.borrow().window_type(), WindowType::GLFW) {
                    error!("ImGui Layer not implemented for this window");
                    panic!("ImGui Layer not implemented for this window");
                }

                let gl = self
                    .window
                    .borrow()
                    .as_any()
                    .downcast_ref::<GLFWWindow>()
                    .expect("Window is not GLFW Window")
                    .gl_context()
                    .expect("GL context missing");
                let renderer = Renderer::initialize(&gl, &mut imgui, &mut self.texture_map, false)
                    .expect("Failed to create ImGui Glow Renderer");

                self.glow_renderer = Some(renderer);
                self.gl = Some(gl);
            }
            GPUContext::None => panic!("To use ImGui Layer need a graphics API"),
        }

        self.imgui = Some(imgui);
    }

    fn on_detach(&mut self) {
        if let (Some(renderer), Some(gl)) = (self.glow_renderer.as_mut(), self.gl.as_ref()) {
            renderer.destroy(&**gl);
        }
        self.glow_renderer = None;
        self.imgui = None;
        self.gl = None;
    }

    fn on_update(&mut self, _input_manager: &dyn Input) {
        let (Some(imgui), Some(renderer), Some(gl)) = (
            self.imgui.as_mut(),
            self.glow_renderer.as_mut(),
            self.gl.as_ref(),
        ) else {
            return;
        };

        let now = Instant::now();
        let io = imgui.io_mut();
        io.update_delta_time(now - self.last_frame);
        io.display_size = [
            self.window.borrow().window_props().width as f32,
            self.window.borrow().window_props().height as f32,
        ];
        self.last_frame = now;

        let ui = imgui.new_frame();
        let _ = ui.dockspace_over_main_viewport();

        for cb in self.ui_callbacks.iter_mut() {
            cb.render(ui);
        }

        let draw_data = imgui.render();
        if draw_data.draw_lists_count() > 0 {
            renderer
                .render(gl.as_ref(), &self.texture_map, draw_data)
                .expect("imgui render failed");
        }
    }

    fn name(&self) -> &str {
        "Imgui layer"
    }

    fn on_event(&mut self, event: &mut core::event::event::EventCategory) {
        let Some(imgui) = self.imgui.as_mut() else {
            return;
        };
        let io = imgui.io_mut();

        let window = self.window.borrow();
        let width = window.get_width();
        let height = window.get_height();
        io.display_size = [width as f32, height as f32];

        let (fb_w, fb_h) = window.get_framebuffer_size();
        if width > 0 && height > 0 {
            io.display_framebuffer_scale =
                [fb_w as f32 / width as f32, fb_h as f32 / height as f32];
        }

        match event {
            EventCategory::EventCategoryMouse(mouse_event) => match mouse_event {
                MouseEvent::MouseMoveEvent(x, y) => io.add_mouse_pos_event([*x, *y]),
                MouseEvent::MouseButtonPressedEvent(button) => {
                    if let Some(imgui_button) = translate_mouse_button(*button) {
                        io.add_mouse_button_event(imgui_button, true);
                    }
                }
                MouseEvent::MouseButtonReleaseEvent(button) => {
                    if let Some(imgui_button) = translate_mouse_button(*button) {
                        io.add_mouse_button_event(imgui_button, false);
                    }
                }
                MouseEvent::MouseScrollEvent(x_offset, y_offset) => {
                    io.add_mouse_wheel_event([*x_offset, *y_offset]);
                }
            },
            EventCategory::EventCategoryKeyboard(key_event) => match key_event {
                KeyEvent::KeyPressedEvent(key_code, _repeat) => {
                    if let Some(key) = glfw_key_to_imgui(*key_code) {
                        io.add_key_event(key, true);
                    }
                    if let Ok(c) = u8::try_from(*key_code) {
                        let ch = c as char;
                        if !ch.is_control() {
                            io.add_input_character(ch);
                        }
                    }
                }
                KeyEvent::KeyReleaseEvent(key_code) => {
                    if let Some(key) = glfw_key_to_imgui(*key_code) {
                        io.add_key_event(key, false);
                    }
                }
            },
            EventCategory::EventCategoryApplication(app_event) => match app_event {
                core::event::event::ApplicationEvent::WindowResizeEvent(width, height) => {
                    io.display_size = [*width as f32, *height as f32];
                }
                _ => {}
            },
            _ => {}
        }
    }
}

impl ImGuiLayer {
    pub fn add_ui_callback(&mut self, cb: Box<dyn ImGuiAddOn>) {
        self.ui_callbacks.push(cb);
    }
}

fn translate_mouse_button(button: u32) -> Option<imgui::MouseButton> {
    match button {
        0 => Some(imgui::MouseButton::Left),
        1 => Some(imgui::MouseButton::Right),
        2 => Some(imgui::MouseButton::Middle),
        3 => Some(imgui::MouseButton::Extra1),
        4 => Some(imgui::MouseButton::Extra2),
        _ => None,
    }
}

fn glfw_key_to_imgui(glfw_key: u32) -> Option<Key> {
    match glfw_key {
        // Modifiers
        340 | 344 => Some(Key::ModShift),
        341 | 345 => Some(Key::ModCtrl),
        342 | 346 => Some(Key::ModAlt),
        343 | 347 => Some(Key::ModSuper),

        // Navigation & Editing
        256 => Some(Key::Escape),
        257 | 258 => Some(Key::Enter),
        259 => Some(Key::Backspace),
        260 => Some(Key::Insert),
        261 => Some(Key::Delete),
        262 => Some(Key::RightArrow),
        263 => Some(Key::LeftArrow),
        264 => Some(Key::DownArrow),
        265 => Some(Key::UpArrow),
        266 => Some(Key::PageUp),
        267 => Some(Key::PageDown),
        268 => Some(Key::Home),
        269 => Some(Key::End),
        32 => Some(Key::Space),

        // Alpha Keys (A-Z)
        65..=90 => match glfw_key {
            65 => Some(Key::A),
            66 => Some(Key::B),
            67 => Some(Key::C),
            68 => Some(Key::D),
            69 => Some(Key::E),
            70 => Some(Key::F),
            71 => Some(Key::G),
            72 => Some(Key::H),
            73 => Some(Key::I),
            74 => Some(Key::J),
            75 => Some(Key::K),
            76 => Some(Key::L),
            77 => Some(Key::M),
            78 => Some(Key::N),
            79 => Some(Key::O),
            80 => Some(Key::P),
            81 => Some(Key::Q),
            82 => Some(Key::R),
            83 => Some(Key::S),
            84 => Some(Key::T),
            85 => Some(Key::U),
            86 => Some(Key::V),
            87 => Some(Key::W),
            88 => Some(Key::X),
            89 => Some(Key::Y),
            90 => Some(Key::Z),
            _ => None,
        },

        // Number Keys (0-9)
        48..=57 => match glfw_key {
            48 => Some(Key::Alpha0),
            49 => Some(Key::Alpha1),
            50 => Some(Key::Alpha2),
            51 => Some(Key::Alpha3),
            52 => Some(Key::Alpha4),
            53 => Some(Key::Alpha5),
            54 => Some(Key::Alpha6),
            55 => Some(Key::Alpha7),
            56 => Some(Key::Alpha8),
            57 => Some(Key::Alpha9),
            _ => None,
        },

        // Function Keys (F1 - F12)
        290..=301 => match glfw_key {
            290 => Some(Key::F1),
            291 => Some(Key::F2),
            292 => Some(Key::F3),
            293 => Some(Key::F4),
            294 => Some(Key::F5),
            295 => Some(Key::F6),
            296 => Some(Key::F7),
            297 => Some(Key::F8),
            298 => Some(Key::F9),
            299 => Some(Key::F10),
            300 => Some(Key::F11),
            301 => Some(Key::F12),
            _ => None,
        },

        _ => None,
    }
}
