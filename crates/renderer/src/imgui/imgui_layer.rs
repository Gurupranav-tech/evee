use core::{
    GPUContext::{self},
    Layer, error,
    window::Window,
};
use std::{cell::RefCell, rc::Rc, time::Instant};

use imgui::Context as ImGuiContext;
use imgui_glow_renderer::{Renderer, SimpleTextureMap};
use window::glfw_window::GLFWWindow;

pub struct ImGuiLayer {
    window: Rc<RefCell<Box<dyn Window>>>,
    imgui: Option<ImGuiContext>,
    glow_renderer: Option<Renderer>,
    texture_map: SimpleTextureMap,
    last_frame: Instant,
    gl: Option<Rc<glow::Context>>,
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

        io.display_size = [
            self.window.borrow().window_props().width as f32,
            self.window.borrow().window_props().height as f32,
        ];
        imgui.style_mut().use_dark_colors();

        match self.window.borrow().window_props().context {
            GPUContext::Opengl => {
                if self.window.borrow().window_type() != "GLFW" {
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

    fn on_update(&mut self) {
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
        ui.show_demo_window(&mut true);

        let draw_data = imgui.render();
        renderer
            .render(&**gl, &self.texture_map, draw_data)
            .expect("imgui render failed");
    }

    fn on_event(&mut self, event: &mut core::event::event::EventCategory) {}
}
