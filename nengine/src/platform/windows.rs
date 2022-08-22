use windows::{
    core::PCWSTR,
    w,
    Win32::{
        Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, WPARAM},
        System::LibraryLoader::GetModuleHandleW,
        UI::WindowsAndMessaging::{
            CreateWindowExW, DefWindowProcW, DispatchMessageW, GetMessageW, LoadCursorW,
            RegisterClassW, ShowWindow, TranslateMessage, CW_USEDEFAULT, HMENU, IDC_ARROW, MSG,
            SW_SHOWNORMAL, WINDOW_EX_STYLE, WNDCLASSW, WS_OVERLAPPEDWINDOW,
        },
    },
};

use std::ffi::OsString;
use std::{os::windows::ffi::OsStrExt, str::FromStr};

pub struct Window {
    raw_handle: HWND,
}

unsafe extern "system" fn window_proc(
    window: HWND,
    message: u32,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    DefWindowProcW(window, message, w_param, l_param)
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

            Window { raw_handle: window }
        }
    }

    fn show(&self) {
        unsafe {
            ShowWindow(self.raw_handle, SW_SHOWNORMAL);
        }
    }
    
    fn set_event_callback(&mut self, _: fn(crate::Event)) {
        // TODO: Fill this out later on.
    }

    // Todo: needed to add a proper closing mechanism
    fn is_open(&self) -> bool {
        true
    }

    fn poll_events(&mut self) {
        unsafe {
            let mut message: MSG = { Default::default() };
            GetMessageW(&mut message, HWND(0), 0, 0);

            TranslateMessage(&message);
            DispatchMessageW(&message);
        }
    }
}
