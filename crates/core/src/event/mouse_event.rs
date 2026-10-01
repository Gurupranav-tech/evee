#[derive(Debug, Clone)]
pub enum MouseEvent {
    // x, y
    MouseMoveEvent(u32, u32),
    MouseScrollEvent(u32, u32),
    // Mouse Button
    MouseButtonPressedEvent(u32),
    MouseButtonReleaseEvent(u32),
}

impl MouseEvent {
    pub fn to_string(&self) -> String {
        match self {
            Self::MouseMoveEvent(x, y) => format!("Mouse Move Event -> x = {x}, y = {y}"),
            Self::MouseScrollEvent(off_x, off_y) => {
                format!("Mouse Scroll Event -> off_x = {off_x}, off_y = {off_y}")
            }
            Self::MouseButtonPressedEvent(btn) => {
                format!("Mouse Button Pressed Event -> btn = {btn}")
            }
            Self::MouseButtonReleaseEvent(btn) => {
                format!("Mouse Button Release Event -> btn = {btn}")
            }
        }
    }
}
