#[derive(Debug)]
pub enum ApplicationEvent {
    WindowResizeEvent(u32, u32),
    WindowCloseEvent,
    AppUpdateEvent,
    AppRenderEvent,
}

impl ApplicationEvent {
    pub fn get_width(&self) -> Option<u32> {
        if let Self::WindowResizeEvent(width, _) = self {
            Some(*width)
        } else {
            None
        }
    }

    pub fn get_height(&self) -> Option<u32> {
        if let Self::WindowResizeEvent(_, height) = self {
            Some(*height)
        } else {
            None
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::WindowResizeEvent(width, height) => {
                format!("Window Resize Event -> height = {height}, width = {width}")
            }
            Self::WindowCloseEvent => format!("Window Close Event"),
            Self::AppRenderEvent => format!("App Render Event"),
            Self::AppUpdateEvent => format!("App Update Event"),
        }
    }
}
