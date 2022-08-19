use crate::{ffi::xcb, Event, MouseButton};
use std::ffi::c_void;

/// The Linux window.
pub struct Window {
    connection: *mut xcb::xcb_connection_t,
    raw_handle: xcb::xcb_window_t,
    is_open: bool,
    event_callback: Option<fn(Event)>,

    // Atoms
    wm_delete_window_atom: xcb::xcb_atom_t,
}

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

fn translate_xcb_buttons(xcb_button_code: u8) -> MouseButton {
    match xcb_button_code {
        1 => MouseButton::Left,
        2 => MouseButton::Middle,
        3 => MouseButton::Right,
        // 4 and 5 are skipped because XCB uses them to send scroll events.
        6 => MouseButton::Six,
        7 => MouseButton::Seven,
        8 => MouseButton::Eight,
        9 => MouseButton::Nine,
        10 => MouseButton::Ten,
        _ => MouseButton::Left
    }
}

impl super::CrossPlatformWindow for Window {
    /// Creates a new window. Most of the parameters should be self-explanator-
    /// y.
    ///
    /// One thing you need to note is that the `fullscreen` parameter currently
    /// does not work, but fullscreen functionalities will be added later.
    ///
    /// Also note that `width` and `height` will be ignored if `fullscreen` is
    /// set to true.
    fn new(width: u32, height: u32, title: &str, fullscreen: bool) -> Window {
        unsafe {
            let connection = xcb::xcb_connect(std::ptr::null(), std::ptr::null_mut());
            let screen = xcb::xcb_setup_roots_iterator(xcb::xcb_get_setup(connection)).data;

            let events = [xcb::XCB_EVENT_MASK_EXPOSURE | xcb::XCB_EVENT_MASK_BUTTON_PRESS];

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
                xcb::XCB_CW_EVENT_MASK,
                events.as_ptr() as *const c_void,
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
                &wm_delete_window_atom as *const u32 as *const c_void,
            );

            if fullscreen {
                let net_wm_state_atom = get_xcb_atom(connection, "_NET_WM_STATE");
                let net_wm_state_fullscreen_atom =
                    get_xcb_atom(connection, "_NET_WM_STATE_FULLSCREEN");

                xcb::xcb_change_property(
                    connection,
                    xcb::XCB_PROP_MODE_APPEND.try_into().unwrap(),
                    window,
                    net_wm_state_atom,
                    4,
                    32,
                    1,
                    &net_wm_state_fullscreen_atom as *const u32 as *const c_void,
                );
            }

            xcb::xcb_flush(connection);

            Window {
                connection,
                raw_handle: window,
                is_open: true,
                event_callback: None,
                wm_delete_window_atom,
            }
        }
    }

    fn set_event_callback(&mut self, callback: fn(Event)) {
        self.event_callback = Some(callback);
    }

    fn show(&self) {
        unsafe {
            xcb::xcb_map_window(self.connection, self.raw_handle);
            xcb::xcb_flush(self.connection);
        }
    }

    fn is_open(&self) -> bool {
        self.is_open
    }

    fn poll_events(&mut self) {
        unsafe {
            let event = xcb::xcb_poll_for_event(self.connection);

            if event == std::ptr::null_mut() {
                return;
            }

            let response_type = (*event).response_type & !0x80;

            // Internally handled events.
            match response_type as u32 {
                xcb::XCB_CLIENT_MESSAGE => {
                    if (*(event as *mut xcb::xcb_client_message_event_t))
                        .data
                        .data32[0]
                        == self.wm_delete_window_atom
                    {
                        self.is_open = false;
                    }
                }
                _ => (),
            }

            // Broadcasted events.
            let translated_event = match response_type as u32 {
                xcb::XCB_BUTTON_PRESS => {
                    let event = event as *mut xcb::xcb_button_press_event_t;

                    Some(Event::MouseButton {
                        button: translate_xcb_buttons((*event).detail),
                        is_press: true,
                    })
                }
                _ => None,
            };

            if let Some(callback) = self.event_callback {
                if let Some(event) = translated_event {
                    callback(event);
                }
            }
        }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            xcb::xcb_disconnect(self.connection);
        }
    }
}
