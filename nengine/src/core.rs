/// The struct that represents the entire Nengine. Rust doesn't let us use glo-
/// bal variables so we have to use a struct to contain everything. So, it is  
/// empty because we don't have any engine components yet, but soon stuff will
/// show up.
pub struct Nengine {}

impl Nengine {
    /// Initializes the Nengine. Because of how Rust works, this can only do s-
    /// ome of the initialization work. The rest is done by the `init` function
    /// .
    pub fn new() -> Nengine {
        println!("[INFO]: Starting Phase 0 of Engine Initialization");

        return Nengine {};
    }

    pub fn init(&mut self) {
        println!("[INFO]: Starting Phase 1 of Engine Initialization");
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
    Escape,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    GraveAccent,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Zero,
    Hyphen,
    EqualSign,
    Backspace,
    Tab,
    Q,
    W,
    E,
    R,
    T,
    Y,
    U,
    I,
    O,
    P,
    LeftSquareBracket,
    RightSquareBracket,
    BackSlash,
    CapsLock,
    A,
    S,
    D,
    F,
    G,
    H,
    J,
    K,
    L,
    Semicolon,
    Quote,
    Enter,
    LeftShift,
    Z,
    X,
    C,
    V,
    B,
    N,
    M,
    Comma,
    Period,
    ForwardSlash,
    RightShift,
    LeftControl,
    LeftSuper,
    LeftAlt,
    Spacebar,
    RightAlt,
    RightSuper,
    Menu,
    RightControl,
}

/// A list of Mouse buttons. Supports up to ten mouse buttons
pub enum MouseButton {
    Left,
    Right,
    Middle,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
}

/// A library-independent method of representing an external event that the Ne-
/// ngine can receive from it's execution environment.
pub enum Event {
    Key { keycode: Key, is_press: bool },
    MouseButton { button: MouseButton, is_press: bool },
    MouseScroll { x: f64, y: f64 },
    MouseMove { x: f64, y: f64 },
}

use std::ffi::c_void;

#[cfg(target_os = "linux")]
use crate::ffi::xcb;

/// The Linux window.
#[cfg(target_os = "linux")]
pub struct Window {
    connection: *mut xcb::xcb_connection_t,
    raw_handle: xcb::xcb_window_t,
    is_open: bool,
    
    // Atoms
    wm_delete_window_atom: xcb::xcb_atom_t
}

#[cfg(target_os = "linux")]
unsafe fn get_xcb_atom(connection: *mut xcb::xcb_connection_t, name: &str) -> xcb::xcb_atom_t {
    let cookie = xcb::xcb_intern_atom(
        connection,
        0,
        name.len().try_into().unwrap(),
        name.as_ptr() as *const i8,
    );
    let reply = xcb::xcb_intern_atom_reply(connection, cookie, std::ptr::null_mut());
    (*reply).atom
}

#[cfg(target_os = "linux")]
impl Window {
    /// Creates a new window. Most of the parameters should be self-explanator-
    /// y.
    ///
    /// One thing you need to note is that the `fullscreen` parameter currently
    /// does not work, but fullscreen functionalities will be added later.
    pub fn new(width: u32, height: u32, title: &str, _fullscreen: bool) -> Window {
        unsafe {
            let connection = xcb::xcb_connect(std::ptr::null(), std::ptr::null_mut());
            let screen = xcb::xcb_setup_roots_iterator(xcb::xcb_get_setup(connection)).data;

            let window = xcb::xcb_generate_id(connection);
            xcb::xcb_create_window(
                connection,
                xcb::XCB_COPY_FROM_PARENT.try_into().unwrap(),
                window,
                (*screen).root,
                0,
                0,
                width.try_into().unwrap(),
                height.try_into().unwrap(),
                0,
                xcb::XCB_WINDOW_CLASS_INPUT_OUTPUT.try_into().unwrap(),
                (*screen).root_visual,
                0,
                std::ptr::null(),
            );

            xcb::xcb_change_property(
                connection,
                xcb::XCB_PROP_MODE_REPLACE.try_into().unwrap(),
                window,
                xcb::XCB_ATOM_WM_NAME,
                xcb::XCB_ATOM_STRING,
                8,
                title.len().try_into().unwrap(),
                title.as_ptr() as *const c_void,
            );
            
            let wm_protocols_atom = get_xcb_atom(connection, "WM_PROTOCOLS");
            let wm_delete_window_atom = get_xcb_atom(connection, "WM_DELETE_WINDOW");
            
            xcb::xcb_change_property(
                connection,
                xcb::XCB_PROP_MODE_REPLACE.try_into().unwrap(),
                window,
                wm_protocols_atom,
                4,
                32,
                1,
                &wm_delete_window_atom as *const u32 as *const c_void
            );

            xcb::xcb_flush(connection);

            Window {
                connection,
                raw_handle: window,
                is_open: true,
                wm_delete_window_atom
            }
        }
    }
    
    pub fn is_open(&self) -> bool {
        self.is_open
    }
    
    pub fn poll_events(&mut self) {
        unsafe {
            let event = xcb::xcb_poll_for_event(self.connection);
            
            if event == std::ptr::null_mut() {
                return;
            }
            
            let response_type = (*event).response_type & !0x80;
            
            match response_type as u32 {
                xcb::XCB_CLIENT_MESSAGE => {
                    if (*(event as *mut xcb::xcb_client_message_event_t)).data.data32[0] == self.wm_delete_window_atom {
                        self.is_open = false;
                    }
                }
                _ => ()
            }
        }
    }
    
    pub fn show(&self) {
        unsafe {
            xcb::xcb_map_window(self.connection, self.raw_handle);
            xcb::xcb_flush(self.connection);
        }
    }
}

#[cfg(target_os = "linux")]
impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            xcb::xcb_disconnect(self.connection);
        }
    }
}
