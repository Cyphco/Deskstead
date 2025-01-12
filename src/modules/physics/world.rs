use raylib::prelude::*;
use super::component::PhysicsComponent;
use super::forces;
use std::collections::HashMap;
use crate::modules::objects::base::GameObject;

/// Represents the physics simulation world
pub struct PhysicsWorld<T: PhysicsComponent + GameObject> {
    pub objects: HashMap<usize, T>,
    gravity: Vector2,
    air_resistance: f32,
    next_id: usize,
    accumulated_time: f32,
}

impl<T: PhysicsComponent + GameObject> Default for PhysicsWorld<T> {
    fn default() -> Self {
        Self {
            objects: HashMap::new(),
            gravity: Vector2::new(0.0, 400.0), // Reduced gravity
            air_resistance: 0.5,  // Increased air resistance
            next_id: 0,
            accumulated_time: 0.0,
        }
    }
}

impl<T: PhysicsComponent + GameObject> PhysicsWorld<T> {
    /// Creates a new physics world
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a physics object to the world and returns its ID
    pub fn add_object(&mut self, object: T) -> usize {
        let id = self.next_id;
        self.objects.insert(id, object);
        self.next_id += 1;
        id
    }

    /// Gets a reference to a physics object by ID
    pub fn get_object(&self, id: usize) -> Option<&T> {
        self.objects.get(&id)
    }

    /// Gets a mutable reference to a physics object by ID
    pub fn get_object_mut(&mut self, id: usize) -> Option<&mut T> {
        self.objects.get_mut(&id)
    }

    /// Gets a mutable reference to all physics objects
    pub fn get_objects_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.objects.values_mut()
    }

    /// Gets a mutable iterator over all objects with their IDs
    pub fn get_objects_with_id_mut(&mut self) -> impl Iterator<Item = (usize, &mut T)> {
        self.objects.iter_mut().map(|(&id, obj)| (id, obj))
    }

    /// Updates the physics simulation
    pub fn update(&mut self, dt: f32) {
        let object_ids: Vec<usize> = self.objects.keys().cloned().collect();
        
        // Update non-fixed objects
        for &id in &object_ids {
            if let Some(obj) = self.objects.get_mut(&id) {
                let body = obj.get_physics_body_mut();
                if !body.fixed {
                    // Apply gravity
                    body.velocity.y += 1000.0 * dt;  // Stronger gravity
                    
                    // Update position
                    body.position.x += body.velocity.x * dt;
                    body.position.y += body.velocity.y * dt;
                }
            }
        }

        // Collect collision info
        let mut collision_info = Vec::new();
        for &id in &object_ids {
            if let Some(obj) = self.objects.get(&id) {
                let body = obj.get_physics_body();
                if body.fixed {
                    // This is a static object (like floor)
                    let static_pos = body.position;
                    let static_size = body.size;
                    let static_restitution = body.restitution;

                    // Calculate static object bounds
                    let static_top = static_pos.y - static_size.y/2.0;
                    let static_bottom = static_pos.y + static_size.y/2.0;
                    let static_left = static_pos.x - static_size.x/2.0;
                    let static_right = static_pos.x + static_size.x/2.0;

                    // Check against all other objects
                    for &other_id in &object_ids {
                        if id != other_id {
                            if let Some(dynamic_obj) = self.objects.get(&other_id) {
                                let dynamic_body = dynamic_obj.get_physics_body();
                                if !dynamic_body.fixed {
                                    // Calculate dynamic object bounds
                                    let dynamic_top = dynamic_body.position.y - dynamic_body.size.y/2.0;
                                    let dynamic_bottom = dynamic_body.position.y + dynamic_body.size.y/2.0;
                                    let dynamic_left = dynamic_body.position.x - dynamic_body.size.x/2.0;
                                    let dynamic_right = dynamic_body.position.x + dynamic_body.size.x/2.0;

                                    // Check for collision
                                    if dynamic_right > static_left && dynamic_left < static_right &&
                                       dynamic_bottom > static_top && dynamic_top < static_bottom {
                                        // Calculate overlap
                                        let overlap_left = dynamic_right - static_left;
                                        let overlap_right = static_right - dynamic_left;
                                        let overlap_top = dynamic_bottom - static_top;
                                        let overlap_bottom = static_bottom - dynamic_top;

                                        // Find smallest overlap
                                        let min_overlap = overlap_left
                                            .min(overlap_right)
                                            .min(overlap_top)
                                            .min(overlap_bottom);

                                        if min_overlap == overlap_top && dynamic_body.velocity.y > 0.0 {
                                            collision_info.push((
                                                other_id,
                                                static_top - dynamic_body.size.y/2.0,
                                                static_restitution * dynamic_body.restitution,
                                                dynamic_body.velocity.clone(),
                                            ));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Apply collision resolutions
        for (id, new_y, restitution, velocity) in collision_info {
            if let Some(obj) = self.objects.get_mut(&id) {
                let body = obj.get_physics_body_mut();
                body.position.y = new_y;
                body.velocity.y = -velocity.y * restitution;
                body.velocity.x *= 0.95; // Friction
            }
        }

        // Sync all objects
        for object in self.objects.values_mut() {
            object.sync_with_physics();
        }
    }

    /// Draws all physics objects
    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        for obj in self.objects.values() {
            obj.draw(d);
        }
    }
}
