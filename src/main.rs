mod modules;

use raylib::prelude::*;
use modules::window_handler::*;


fn main() {
    let (width,height) = get_screen_dimensions();



    let (mut main_window_handle, thread) = raylib::init()
        .size(width,height)
        .title("Transparent Undecorated Window")
        .undecorated()
        .transparent()
        .click_through()
        .always_top()
        .vsync()
        .msaa_4x()
        .unfocused()
        .build();

    let hwnd = get_hwnd(&mut main_window_handle);
    
    hide_taskbar_icon(hwnd);


    while !main_window_handle.window_should_close() {
        let mut main_window_thread =  main_window_handle.begin_drawing(&thread);


        main_window_thread.clear_background(Color::BLANK);

        // Get the global cursor position
        let cursor_pos = get_global_mouse_position();

        // Draw a circle at the cursor position
        main_window_thread.draw_circle(cursor_pos.0, cursor_pos.1, 20.0f32, Color::GREEN);
    }


}


