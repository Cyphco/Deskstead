// Function to get the screen dimensions

use winapi::um::winuser::{
    GetSystemMetrics, GetCursorPos, GetAsyncKeyState,
    SM_CXSCREEN, SM_CYSCREEN,
    VK_LBUTTON, VK_RBUTTON, VK_MBUTTON,
};
use winapi::shared::windef::POINT;
use raylib::prelude::*;

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

pub trait Vector2Ext {
    fn scale_by(&self, scale: f32) -> Vector2;
}

impl Vector2Ext for Vector2 {
    fn scale_by(&self, scale: f32) -> Vector2 {
        Vector2::new(self.x * scale, self.y * scale)
    }
}
