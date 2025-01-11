mod modules;

use raylib::prelude::*;
use modules::custom_functions::*;
use modules::init::initialize_window;
use modules::objects::base::GameObject;
use modules::objects::textured::TexturedObject;

fn main() {
    
    let window_init = initialize_window();
    let mut main_window = window_init.0;
    let main_thread = window_init.1;

    let exit_flag = 0;

    let mut logo_object = TexturedObject::new(&mut main_window, &main_thread, "assets/logo.png", 500.0, 500.0, 0.1, 1.0);

    while exit_flag != 1 {
        
        let mut draw_handler = main_window.begin_drawing(&main_thread);

        draw_handler.clear_background(Color::BLANK);
       
        let cursor_pos = get_global_mouse_position();
        draw_handler.draw_circle(cursor_pos.0, cursor_pos.1, 60.0f32, Color::new(10, 0, 0, 100));

        logo_object.draw(&mut draw_handler);
        logo_object.set_rotation(logo_object.get_rotation() + 1.0);

        
    }

}
