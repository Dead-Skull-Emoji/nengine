use windows::{
    core::PCWSTR,
    w,
    Win32::{
        Foundation::{HWND, LPARAM, LRESULT, WPARAM},
        System::LibraryLoader::GetModuleHandleW,
        UI::WindowsAndMessaging::{
            CreateWindowExW, DefWindowProcW, RegisterClassW, CW_USEDEFAULT, HMENU, WINDOW_EX_STYLE,
            WNDCLASSW, WS_OVERLAPPEDWINDOW, ShowWindow, SW_SHOWNORMAL, GetMessageW, MSG, TranslateMessage, DispatchMessageW
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

impl Window {
    pub fn new(width: u32, height: u32, title: &str, _fullscreen: bool) -> Result<Window, ()> {
        unsafe {
            let class_name = w!("NENGINE_WINDOW_CLASS");

            let window_class = WNDCLASSW {
                hInstance: GetModuleHandleW(None).unwrap(),
                lpszClassName: PCWSTR::from(class_name),
                lpfnWndProc: Some(window_proc),
                ..Default::default()
            };

            RegisterClassW(&window_class);

            let title = OsString::from_str(title).unwrap();
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
                GetModuleHandleW(None).unwrap(),
                std::ptr::null(),
            );
            
            if window == HWND(0) {
                return Err(());
            }

            Ok(Window {
                raw_handle: window
            })
        }
    }
    
    pub fn show(&self) {
        unsafe {
            ShowWindow(self.raw_handle, SW_SHOWNORMAL);
        }
    }
    
    // Todo: needed to add a proper closing mechanism
    pub fn is_open(&self) -> bool {
        true
    }
    
    pub fn poll_events(&self) {
        unsafe {
            let mut message: MSG = { Default::default() };
            GetMessageW(&mut message, HWND(0), 0, 0);
            
            TranslateMessage(&message);
            DispatchMessageW(&message);
        }
    }
}
