#[derive(Debug)]
pub enum KeyEvent {
    // key_code, repeat_count
    KeyPressedEvent(u32, u32),
    KeyReleaseEvent(u32),
}

impl KeyEvent {
    pub fn get_repeat_count(&self) -> Option<u32> {
        if let Self::KeyPressedEvent(_, repeat_count) = self {
            Some(*repeat_count)
        } else {
            None
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::KeyPressedEvent(keycode, repeat_count) => {
                format!("Key Pressed Event -> keycode = {keycode}, repeat_count = {repeat_count}")
            }
            Self::KeyReleaseEvent(keycode) => format!("Key Release Event -> keycode = {keycode}"),
        }
    }
}
