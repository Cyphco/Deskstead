use raylib::prelude::*;
use modules::{
    custom_functions::{get_global_mouse_position, get_global_mouse_state, get_screen_dimensions, get_taskbar_height}, 
    init::initialize_window, 
    window_setting_override::hide_taskbar_icon
};


mod modules;


fn main() {
    let window_init = initialize_window();
    let mut main_window = window_init.0;
    let thread = window_init.1;

    hide_taskbar_icon(&mut main_window);
   

    while !main_window.window_should_close() {

        // Draw
        let mut d = main_window.begin_drawing(&thread);
        d.clear_background(Color::BLANK);
        
    }
}
