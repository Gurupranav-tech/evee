pub use crate::event::{
    application_event::ApplicationEvent, key_event::KeyEvent, mouse_event::MouseEvent,
};

#[derive(Debug)]
pub enum EventCategory {
    None,
    EventCategoryApplication(ApplicationEvent),
    EventCategoryKeyboard(KeyEvent),
    EventCategoryMouse(MouseEvent),
}
