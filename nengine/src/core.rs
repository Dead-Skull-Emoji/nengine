/// The struct that represents the entire Nengine. Rust doesn't let us use glo-
/// bal variables so we have to use a struct to contain everything. So, it is  
/// empty because we don't have any engine components yet, but soon stuff will 
/// show up.
pub struct Nengine {}

impl Nengine {
    /// Initializes the Nengine. This does all of the initialization work befo-
    /// re transferring the ownership to a struct that it will return. In the  
    /// future one of it's parameters would be a deserialized scene.
    pub fn new() -> Nengine {
        println!("[INFO]: Initialized the Nengine.");

        return Nengine {};
    }
    
    /// Returns if the Nengine is still running.
    pub fn is_running(&self) -> bool {
        return true;
    }
    
    fn on_event(&self, event: Event) {
        if let Event::Key { keycode, is_press } = event {
            if let Key::W = keycode {
                if is_press {
                    println!("The W key has been pressed!");
                }
            }
        }
    }
    
    /// Executed every frame, this functions performs all of the per-frame ope-
    /// rations.
    pub fn update(&mut self) {
        println!("[INFO]: Updating the Nengine.");
    }
}

impl Drop for Nengine {
    fn drop(&mut self) {
        println!("[INFO]: Shutting down the Nengine.");
    }
}

/// A list of Keycodes (or rather not keycodes since Rust enums do not represe-
/// nt integers). Incomplete for now and does not support Apple keyboards.
pub enum Key {
    Escape, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,
    GraveAccent, One, Two, Three, Four, Five, Six, Seven, Eight, Nine, Zero, Hyphen, EqualSign, Backspace,
    Tab, Q, W, E, R, T, Y, U, I, O, P, LeftSquareBracket, RightSquareBracket, BackSlash,
    CapsLock, A, S, D, F, G, H, J, K, L, Semicolon, Quote, Enter,
    LeftShift, Z, X, C, V, B, N, M, Comma, Period, ForwardSlash, RightShift,
    LeftControl, LeftSuper, LeftAlt, Spacebar, RightAlt, RightSuper, Menu, RightControl
}

/// A list of Mouse buttons. Supports up to ten mouse buttons
pub enum MouseButton {
    Left, Right, Middle,
    One, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten
}

/// A library-independent method of representing an external event that the Ne-
/// ngine can receive from it's execution environment.
pub enum Event {
    Key { keycode: Key, is_press: bool },
    MouseButton { button: MouseButton, is_press: bool },
    MouseScroll { x: f64, y: f64 },
    MouseMove { x: f64, y: f64 }
}