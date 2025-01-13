// Function to get the screen dimensions

use winapi::um::winuser::{
    GetSystemMetrics, GetCursorPos, GetAsyncKeyState, SystemParametersInfoW,
    SM_CXSCREEN, SM_CYSCREEN, SM_CYVIRTUALSCREEN,
    VK_LBUTTON, VK_RBUTTON, VK_MBUTTON, SPI_GETWORKAREA,
};
use winapi::shared::windef::{POINT, RECT};
use winapi::ctypes::c_void;


/// Gets the current screen dimensions
pub fn get_screen_dimensions() -> (i32, i32) {
    unsafe {
        let width = GetSystemMetrics(SM_CXSCREEN);
        let height = GetSystemMetrics(SM_CYSCREEN);
        (width, height)
    }
}

/// Gets the current cursor position relative to the screen
pub fn get_global_mouse_position() -> (i32, i32) {
    unsafe {
        let mut point = POINT { x: 0, y: 0 };
        GetCursorPos(&mut point);
        (point.x, point.y)
    }
}

/// Gets the current mouse button states
pub fn get_global_mouse_state() -> (bool, bool, bool) {
    unsafe {
        (
            (GetAsyncKeyState(VK_LBUTTON as i32) & 0x8000u16 as i16) != 0,
            (GetAsyncKeyState(VK_RBUTTON as i32) & 0x8000u16 as i16) != 0,
            (GetAsyncKeyState(VK_MBUTTON as i32) & 0x8000u16 as i16) != 0,
        )
    }
}

/// Gets the height of the Windows taskbar
pub fn get_taskbar_height() -> f32 {
    unsafe {
        let mut work_area: RECT = std::mem::zeroed();
        let screen_height = GetSystemMetrics(SM_CYSCREEN);
        
        if SystemParametersInfoW(
            SPI_GETWORKAREA,
            0,
            &mut work_area as *mut RECT as *mut c_void,
            0,
        ) == 0 {
            // Failed to get work area, return a reasonable default
            48.0 // Default Windows 11 taskbar height
        } else {
            // Calculate taskbar height as difference between screen height and work area
            (screen_height - (work_area.bottom - work_area.top)).abs() as f32
        }
    }
}

