use crate::event::event::EventCategory;
use std::error::Error;

pub struct WindowProps {
    pub title: String,
    pub width: u32,
    pub height: u32,
}

pub trait Window {
    fn new(props: WindowProps) -> Result<Box<dyn Window>, Box<dyn Error>>
    where
        Self: Sized;

    fn on_update(&mut self);
    fn get_width(&self) -> u32;
    fn get_height(&self) -> u32;

    fn set_event_callback(&mut self, callback: Box<dyn FnMut(EventCategory)>);
    fn set_vsync(&mut self, enabled: bool);
    fn is_vsync(&self) -> bool;
}
