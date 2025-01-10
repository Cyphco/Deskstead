

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
pub fn hide_taskbar_icon(handle: &mut RaylibHandle) {
    let hwnd = get_hwnd(handle);
    unsafe {
        let ex_style = GetWindowLongPtrW(hwnd, GWL_EXSTYLE) as isize;
        SetWindowLongPtrW(hwnd, GWL_EXSTYLE, (ex_style & !(WS_EX_APPWINDOW as isize)) | (WS_EX_TOOLWINDOW as isize));
    }
}


