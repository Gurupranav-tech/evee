use crate::{layer_stack::Layer, window::Window};
use std::{cell::RefCell, error::Error, rc::Rc};

pub trait Application {
    fn create_window(&mut self) -> Result<Rc<RefCell<Box<dyn Window>>>, Box<dyn Error>>;

    fn create_layers(&self, _window: Rc<RefCell<Box<dyn Window>>>) -> Vec<Box<dyn Layer>> {
        vec![]
    }

    fn create_overlays(&self, _window: Rc<RefCell<Box<dyn Window>>>) -> Vec<Box<dyn Layer>> {
        vec![]
    }

    fn clear(&self) {}

    fn set_clear_color(&self, _color: glam::Vec4) {}
}
