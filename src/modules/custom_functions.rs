
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
