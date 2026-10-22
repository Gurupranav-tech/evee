pub trait Input {
    fn is_keypressed(&self, _keycode: KeyCode) -> bool {
        false
    }

    fn is_mousebtn_pressed(&self, _btn: MouseButton) -> bool {
        false
    }

    fn get_mouse_position(&self) -> (f64, f64) {
        (0.0, 0.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum KeyCode {
    Unknown = -1,

    // Printable Keys
    Space = 32,
    Apostrophe = 39,
    Comma = 44,
    Minus = 45,
    Period = 46,
    Slash = 47,
    Num0 = 48,
    Num1 = 49,
    Num2 = 50,
    Num3 = 51,
    Num4 = 52,
    Num5 = 53,
    Num6 = 54,
    Num7 = 55,
    Num8 = 56,
    Num9 = 57,
    Semicolon = 59,
    Equal = 61,
    A = 65,
    B = 66,
    C = 67,
    D = 68,
    E = 69,
    F = 70,
    G = 71,
    H = 72,
    I = 73,
    J = 74,
    K = 75,
    L = 76,
    M = 77,
    N = 78,
    O = 79,
    P = 80,
    Q = 81,
    R = 82,
    S = 83,
    T = 84,
    U = 85,
    V = 86,
    W = 87,
    X = 88,
    Y = 89,
    Z = 90,
    LeftBracket = 91,
    Backslash = 92,
    RightBracket = 93,
    GraveAccent = 96,
    World1 = 161,
    World2 = 162,

    // Function Keys / Navigation
    Escape = 256,
    Enter = 257,
    Tab = 258,
    Backspace = 259,
    Insert = 260,
    Delete = 261,
    Right = 262,
    Left = 263,
    Down = 264,
    Up = 265,
    PageUp = 266,
    PageDown = 267,
    Home = 268,
    End = 269,
    CapsLock = 280,
    ScrollLock = 281,
    NumLock = 282,
    PrintScreen = 283,
    Pause = 284,
    F1 = 290,
    F2 = 291,
    F3 = 292,
    F4 = 293,
    F5 = 294,
    F6 = 295,
    F7 = 296,
    F8 = 297,
    F9 = 298,
    F10 = 299,
    F11 = 300,
    F12 = 301,
    F13 = 302,
    F14 = 303,
    F15 = 304,
    F16 = 305,
    F17 = 306,
    F18 = 307,
    F19 = 308,
    F20 = 309,
    F21 = 310,
    F22 = 311,
    F23 = 312,
    F24 = 313,
    F25 = 314,

    // Keypad
    Kp0 = 320,
    Kp1 = 321,
    Kp2 = 322,
    Kp3 = 323,
    Kp4 = 324,
    Kp5 = 325,
    Kp6 = 326,
    Kp7 = 327,
    Kp8 = 328,
    Kp9 = 329,
    KpDecimal = 330,
    KpDivide = 331,
    KpMultiply = 332,
    KpSubtract = 333,
    KpAdd = 334,
    KpEnter = 335,
    KpEqual = 336,

    // Modifiers
    LeftShift = 340,
    LeftControl = 341,
    LeftAlt = 342,
    LeftSuper = 343,
    RightShift = 344,
    RightControl = 345,
    RightAlt = 346,
    RightSuper = 347,
    Menu = 348,
}

// Bitflags for Modifier Keys
pub struct KeyModifiers;

impl KeyModifiers {
    pub const SHIFT: u16 = 0x0001;
    pub const CONTROL: u16 = 0x0002;
    pub const ALT: u16 = 0x0004;
    pub const SUPER: u16 = 0x0008;
    pub const CAPS_LOCK: u16 = 0x0010;
    pub const NUM_LOCK: u16 = 0x0020;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum JoystickId {
    Joystick1 = 0,
    Joystick2 = 1,
    Joystick3 = 2,
    Joystick4 = 3,
    Joystick5 = 4,
    Joystick6 = 5,
    Joystick7 = 6,
    Joystick8 = 7,
    Joystick9 = 8,
    Joystick10 = 9,
    Joystick11 = 10,
    Joystick12 = 11,
    Joystick13 = 12,
    Joystick14 = 13,
    Joystick15 = 14,
    Joystick16 = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum MouseButton {
    Button1 = 0,
    Button2 = 1,
    Button3 = 2,
    Button4 = 3,
    Button5 = 4,
    Button6 = 5,
    Button7 = 6,
    Button8 = 7,
}

impl MouseButton {
    pub const LEFT: MouseButton = MouseButton::Button1;
    pub const RIGHT: MouseButton = MouseButton::Button2;
    pub const MIDDLE: MouseButton = MouseButton::Button3;
    pub const LAST: MouseButton = MouseButton::Button8;
}

pub struct JoystickHat;

impl JoystickHat {
    pub const CENTERED: u8 = 0x0000;
    pub const UP: u8 = 0x0001;
    pub const RIGHT: u8 = 0x0002;
    pub const DOWN: u8 = 0x0004;
    pub const LEFT: u8 = 0x0008;
    pub const RIGHT_UP: u8 = Self::RIGHT | Self::UP;
    pub const RIGHT_DOWN: u8 = Self::RIGHT | Self::DOWN;
    pub const LEFT_UP: u8 = Self::LEFT | Self::UP;
    pub const LEFT_DOWN: u8 = Self::LEFT | Self::DOWN;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum GamepadButton {
    A = 0,
    B = 1,
    X = 2,
    Y = 3,
    LeftBumper = 4,
    RightBumper = 5,
    Back = 6,
    Start = 7,
    Guide = 8,
    LeftThumb = 9,
    RightThumb = 10,
    DpadUp = 11,
    DpadRight = 12,
    DpadDown = 13,
    DpadLeft = 14,
}

impl GamepadButton {
    pub const CROSS: GamepadButton = GamepadButton::A;
    pub const CIRCLE: GamepadButton = GamepadButton::B;
    pub const SQUARE: GamepadButton = GamepadButton::X;
    pub const TRIANGLE: GamepadButton = GamepadButton::Y;
    pub const LAST: GamepadButton = GamepadButton::DpadLeft;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum GamepadAxis {
    LeftX = 0,
    LeftY = 1,
    RightX = 2,
    RightY = 3,
    LeftTrigger = 4,
    RightTrigger = 5,
}

impl GamepadAxis {
    pub const LAST: GamepadAxis = GamepadAxis::RightTrigger;
}
