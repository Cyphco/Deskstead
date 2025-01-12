use raylib::prelude::*;
use modules::{
    custom_functions::{get_global_mouse_position, get_global_mouse_state, get_screen_dimensions, Vector2Ext}, 
    objects::{PhysicsObjectBuilder, base::GameObject}, 
    physics::world::PhysicsWorld, 
    window_setting_override::hide_taskbar_icon
};

mod modules;


fn main() {
    let (mut main_window, thread) = raylib::init()
        .size(get_screen_dimensions().0, get_screen_dimensions().1)
        .always_top()
        .borderless()
        .click_through()
        .transparent()
        .undecorated()
        .unfocused()
        .build();

    hide_taskbar_icon(&mut main_window);
    let logo_texture = main_window.load_texture(&thread, "assets/logo.png").expect("Couldn't load logo.png");
    let screen_height = get_screen_dimensions().1 as f32;
    let floor_y = screen_height - 100.0;

    // Create falling object using builder
    let test_rec = PhysicsObjectBuilder::new()
        .with_texture(logo_texture)
        .at_position(Vector2::new(100.0, 50.0))
        .with_size(Vector2::new(50.0, 50.0))
        .with_mass(100.0)
        .with_restitution(0.7)
        .with_friction(0.1)  // Make it bouncy
        .build();
    
    // Create floor using builder
    let floor_texture = main_window.load_texture(&thread, "assets/water.png").expect("Couldn't load water.png");
    let floor = PhysicsObjectBuilder::new()
        .with_texture(floor_texture)
        .at_position(Vector2::new(get_screen_dimensions().0 as f32 / 2.0, floor_y))
        .with_size(Vector2::new(get_screen_dimensions().0 as f32, 50.0))
        .fixed()
        .with_restitution(0.1)
        .with_friction(0.1)  // Make it static
        .build();

    let mut physics_world = PhysicsWorld::new();
    let test_rec_id = physics_world.add_object(test_rec);
    physics_world.add_object(floor);

    let mut held_object: Option<usize> = None;
    let mut grab_offset: Option<Vector2> = None;
    let mut last_mouse_pos = Vector2::zero();
    let mut last_delta = Vector2::zero();

    while !main_window.window_should_close() {
        let dt = main_window.get_frame_time();
        
        // Update physics
        physics_world.update(dt);

        // Handle mouse interaction
        let (left_click, _, _) = get_global_mouse_state();
        let (mouse_x, mouse_y) = get_global_mouse_position();
        let mouse_pos = Vector2::new(mouse_x as f32, mouse_y as f32);
        let mouse_delta = mouse_pos - last_mouse_pos;
        last_mouse_pos = mouse_pos;

        if left_click {
            if let Some(held_id) = held_object {
                // Update held object
                if let Some(obj) = physics_world.get_object_mut(held_id) {
                    if let Some(offset) = grab_offset {
                        let target_pos = mouse_pos - offset;
                        let current_pos = obj.get_position();
                        let delta = target_pos - current_pos;
                        last_delta = Vector2::new(delta.x * 15.0, delta.y * 15.0);
                        obj.set_velocity(last_delta);
                    }
                } else {
                    // Object was removed or invalid
                    held_object = None;
                    grab_offset = None;
                }
            } else {
                // Try to grab an object
                for (id, obj) in physics_world.get_objects_with_id_mut() {
                    let obj_pos = obj.get_position();
                    let obj_size = obj.get_size();
                    
                    // Check if mouse is within object bounds
                    let half_width = obj_size.x / 2.0;
                    let half_height = obj_size.y / 2.0;
                    
                    if (mouse_x as f32 - obj_pos.x).abs() < half_width && 
                       (mouse_y as f32 - obj_pos.y).abs() < half_height {
                        held_object = Some(id);
                        grab_offset = Some(mouse_pos - obj_pos);
                        last_delta = Vector2::zero();
                        break;
                    }
                }
            }
        } else {
            // Release object with momentum
            if held_object.is_some() {
                if let Some(obj) = physics_world.get_object_mut(held_object.unwrap()) {
                    // Apply momentum based on the last movement velocity
                    let release_velocity = last_delta + mouse_delta.scale_by(5.0);
                    obj.set_velocity(release_velocity);
                }
            }
            held_object = None;
            grab_offset = None;
        }

        // Draw
        let mut d = main_window.begin_drawing(&thread);
        d.clear_background(Color::BLANK);
        physics_world.draw(&mut d);
    }
}
