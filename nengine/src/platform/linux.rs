use std::ffi::c_void;
use crate::ffi::xcb;

/// The Linux window.
pub struct Window {
    connection: *mut xcb::xcb_connection_t,
    raw_handle: xcb::xcb_window_t,
    is_open: bool,
    
    // Atoms
    wm_delete_window_atom: xcb::xcb_atom_t
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

impl super::CrossPlatformWindow for Window {
    /// Creates a new window. Most of the parameters should be self-explanator-
    /// y.
    ///
    /// One thing you need to note is that the `fullscreen` parameter currently
    /// does not work, but fullscreen functionalities will be added later.
    fn new(width: u32, height: u32, title: &str, _fullscreen: bool) -> Window {
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
    
    fn show(&self) {
        unsafe {
            xcb::xcb_map_window(self.connection, self.raw_handle);
            xcb::xcb_flush(self.connection);
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