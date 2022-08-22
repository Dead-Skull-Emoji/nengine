use windows::{
    core::PCWSTR,
    w,
    Win32::{
        Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, WPARAM},
        System::LibraryLoader::GetModuleHandleW,
        UI::WindowsAndMessaging::{
            CreateWindowExW, DefWindowProcW, LoadCursorW, PeekMessageW, RegisterClassW, ShowWindow,
            CW_USEDEFAULT, HMENU, IDC_ARROW, MSG, PM_REMOVE, SW_SHOWNORMAL, WINDOW_EX_STYLE,
            WM_QUIT, WNDCLASSW, WS_OVERLAPPEDWINDOW, TranslateMessage, DispatchMessageW, PostQuitMessage, WM_CLOSE,
        },
    },
};

use std::ffi::OsString;
use std::{os::windows::ffi::OsStrExt, str::FromStr};

pub struct Window {
    raw_handle: HWND,
    is_open: bool,
}

unsafe extern "system" fn window_proc(
    window: HWND,
    message: u32,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    match message {
        WM_CLOSE => {
            PostQuitMessage(0);
            windows::Win32::Foundation::LRESULT(0)
        }
        _ => DefWindowProcW(window, message, w_param, l_param)
    }
}

impl super::CrossPlatformWindow for Window {
    fn new(width: u32, height: u32, title: &str, _fullscreen: bool) -> Window {
        unsafe {
            let class_name = w!("NENGINE_WINDOW_CLASS");

            let h_instance = GetModuleHandleW(None).unwrap();
            let window_class = WNDCLASSW {
                hInstance: h_instance,
                lpszClassName: PCWSTR::from(class_name),
                lpfnWndProc: Some(window_proc),
                hCursor: LoadCursorW(HINSTANCE(0), IDC_ARROW).unwrap(),
                ..Default::default()
            };

            RegisterClassW(&window_class);

            let title = title.to_owned() + "\0";
            let title = OsString::from_str(title.as_str()).unwrap();
            let title = title.as_os_str();
            let title = title.encode_wide();
            let title: Vec<u16> = title.collect();

            let window = CreateWindowExW(
                WINDOW_EX_STYLE::default(),
                class_name,
                PCWSTR::from_raw(title.as_ptr()),
                WS_OVERLAPPEDWINDOW,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                width.try_into().unwrap(),
                height.try_into().unwrap(),
                HWND::default(),
                HMENU::default(),
                h_instance,
                std::ptr::null(),
            );

            Window {
                raw_handle: window,
                is_open: true,
            }
        }
    }

    fn set_event_callback(&mut self, _: fn(crate::Event)) {
        // TODO: Fill this out later on.
    }

    fn show(&self) {
        unsafe {
            ShowWindow(self.raw_handle, SW_SHOWNORMAL);
        }
    }

    // Todo: needed to add a proper closing mechanism
    fn is_open(&self) -> bool {
        self.is_open
    }

    fn poll_events(&mut self) {
        unsafe {
            let mut message: MSG = { Default::default() };
            while PeekMessageW(&mut message, self.raw_handle, 0, 0, PM_REMOVE).as_bool() {
                if message.message == WM_QUIT {
                    self.is_open = false;
                } else {
                    TranslateMessage(&message);
                    DispatchMessageW(&message);
                }
            }
        }
    }
}
