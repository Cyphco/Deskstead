mod modules;

use raylib::prelude::*;
use modules::window_setting_override::*;
use modules::custom_functions::*;


fn main() {
  
    let (mut main_window_handle, thread) = raylib::init()
        .size(get_screen_dimensions().0,get_screen_dimensions().1)
        .title("Transparent Undecorated Window")
        .undecorated()
        .transparent()
        .click_through()
        .always_top()
        .vsync()
        .msaa_4x()
        .unfocused()
        .build();

    
    
    hide_taskbar_icon(&mut main_window_handle);


    while !main_window_handle.window_should_close() {
        let mut main_window_thread =  main_window_handle.begin_drawing(&thread);


        main_window_thread.clear_background(Color::BLANK);

        // Get the global cursor position
        let cursor_pos = get_global_mouse_position();

        // Draw a circle at the cursor position
        main_window_thread.draw_circle(cursor_pos.0, cursor_pos.1, 20.0f32, Color::GREEN);
    }


}


