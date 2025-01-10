





//Function to get the screen dimensions

use winapi::um::winuser::{SM_CXSCREEN, SM_CYSCREEN};
pub fn get_screen_dimensions() -> (i32, i32) {
    unsafe {
        let width = GetSystemMetrics(SM_CXSCREEN);
        let height = GetSystemMetrics(SM_CYSCREEN);
        (width, height)
    }
}


// Function to get the global mouse position

use winapi::um::winuser::{GetCursorPos, GetSystemMetrics,};
use winapi::shared::windef::POINT;
pub fn get_global_mouse_position() -> (i32, i32) {

    unsafe {
        let mut point: POINT = POINT { x: 0, y: 0 };
        GetCursorPos(&mut point);
        let cursor_pos = (point.x, point.y);
        //println!("Global cursor position: x={}, y={}", cursor_pos.0, cursor_pos.1);
        cursor_pos
    }
}




//Get the window handle
use winapi::shared::windef::{HWND, HWND__};
use raylib::prelude::RaylibHandle;
pub fn get_hwnd(handle: &mut RaylibHandle) -> HWND {
    unsafe {
        let hwnd = handle.get_window_handle() as *mut HWND__;
        hwnd
    }
}


use winapi::um::winuser::{GetWindowLongPtrW, SetWindowLongPtrW, GWL_EXSTYLE, WS_EX_TOOLWINDOW, WS_EX_APPWINDOW};
pub fn hide_taskbar_icon(hwnd: HWND) {
    unsafe {
        let ex_style = GetWindowLongPtrW(hwnd, GWL_EXSTYLE) as isize;
        SetWindowLongPtrW(hwnd, GWL_EXSTYLE, (ex_style & !(WS_EX_APPWINDOW as isize)) | (WS_EX_TOOLWINDOW as isize));
    }
}


