mod modules;

use ffi::DrawCircle;
use raylib::prelude::*;
use modules::custom_functions::*;
use modules::init::initialize_window;
use modules::objects::base::GameObject;
use modules::objects::particle::ParticleSystem;

fn main() {
    
    let window_init = initialize_window();
    let mut main_window = window_init.0;
    let main_thread = window_init.1;

    //Initialize curser pos
    let cursor_pos = get_global_mouse_position();
    let cursor_vec = Vector2::new(cursor_pos.0 as f32, cursor_pos.1 as f32);
    
    // Load the logo texture for particles
    let logo_texture = main_window.load_texture(&main_thread, "assets/logo.png").expect("Couldn't load texture");
    

    

    let mut particle_system = ParticleSystem::new()
        .texture(logo_texture)
        .scale(0.01)
        .emission_rate(100.0)
        .position(Vector2::new(cursor_vec.x,cursor_vec.y))
        .gravity(Vector2::new(0.0,-0.01))
        .angle_range(0.0..45.0)
        .rotation_speed(0.0..0.1)
        .initial_speed(0.0..0.1)  
        .build();

    
    while !main_window.window_should_close() {
        
        let mut draw_handler = main_window.begin_drawing(&main_thread);

        draw_handler.clear_background(Color::BLANK);
       

        let cursor_pos = get_global_mouse_position();
        let cursor_vec = Vector2::new(cursor_pos.0 as f32, cursor_pos.1 as f32);

        // Update and draw the particle system
        particle_system.update();
        particle_system.set_position(cursor_vec);
        particle_system.draw(&mut draw_handler);
        
        

    }

}
