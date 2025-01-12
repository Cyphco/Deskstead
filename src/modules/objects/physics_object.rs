use raylib::prelude::*;
use super::base::GameObject;
use crate::modules::physics::{body::PhysicsBody, component::PhysicsComponent};

/// A wrapper that adds physics behavior to any GameObject
pub struct PhysicsObject<T: GameObject> {
    game_object: T,
    physics: PhysicsBody,
}

impl<T: GameObject> PhysicsObject<T> {
    pub fn new(game_object: T, position: Vector2, size: Vector2, mass: f32, fixed: bool) -> Self {
        let mut physics = PhysicsBody::new(position, mass);
        physics.set_size(size);
        physics.set_fixed(fixed);
        
        let mut obj = Self {
            game_object,
            physics,
        };
        obj.sync_with_physics();
        obj
    }

    /// Get a reference to the wrapped game object
    pub fn get_game_object(&self) -> &T {
        &self.game_object
    }

    /// Get a mutable reference to the wrapped game object
    pub fn get_game_object_mut(&mut self) -> &mut T {
        &mut self.game_object
    }

    /// Apply a force to the physics body
    pub fn apply_force(&mut self, force: Vector2) {
        self.physics.apply_force(force);
    }

    /// Apply an instantaneous impulse (immediate velocity change)
    pub fn apply_impulse(&mut self, impulse: Vector2) {
        self.physics.apply_impulse(impulse);
    }

    /// Set the velocity directly
    pub fn set_velocity(&mut self, velocity: Vector2) {
        self.physics.velocity = velocity;
    }

    /// Get the current velocity
    pub fn get_velocity(&self) -> Vector2 {
        self.physics.velocity
    }

    /// Stop all movement
    pub fn stop(&mut self) {
        self.physics.velocity = Vector2::zero();
        self.physics.acceleration = Vector2::zero();
    }

    /// Apply an upward force (useful for jumping)
    pub fn jump(&mut self, force: f32) {
        self.apply_force(Vector2::new(0.0, -force));
    }

    /// Set the restitution (bounciness) factor
    pub fn set_restitution(&mut self, restitution: f32) {
        self.physics.restitution = restitution.clamp(0.0, 1.0);
    }

    /// Set the friction coefficient
    pub fn set_friction(&mut self, friction: f32) {
        self.physics.friction = friction.max(0.0);
    }

    /// Get whether the object is fixed (static)
    pub fn is_fixed(&self) -> bool {
        self.physics.fixed
    }

    /// Set whether the object is fixed (static)
    pub fn set_fixed(&mut self, fixed: bool) {
        self.physics.set_fixed(fixed);
    }
}

impl<T: GameObject> GameObject for PhysicsObject<T> {
    fn update(&mut self, dt: f32) {
        // Update the wrapped game object
        self.game_object.update(dt);
        
        // Sync the game object's position with physics
        self.game_object.set_position(self.physics.position);
        
        // Optional: Add rotation based on velocity
        let rotation = self.physics.velocity.x * 0.01;
        self.game_object.set_rotation(rotation);
    }

    fn draw(&self, draw_handle: &mut RaylibDrawHandle) {
        self.game_object.draw(draw_handle);
    }

    fn get_position(&self) -> Vector2 {
        self.game_object.get_position()
    }

    fn set_position(&mut self, position: Vector2) {
        self.game_object.set_position(position);
    }

    fn get_rotation(&self) -> f32 {
        self.game_object.get_rotation()
    }

    fn set_rotation(&mut self, rotation: f32) {
        self.game_object.set_rotation(rotation);
    }

    fn get_size(&self) -> Vector2 {
        self.game_object.get_size()
    }
}

impl<T: GameObject> PhysicsComponent for PhysicsObject<T> {
    fn get_physics_body(&self) -> &PhysicsBody {
        &self.physics
    }

    fn get_physics_body_mut(&mut self) -> &mut PhysicsBody {
        &mut self.physics
    }

    fn sync_with_physics(&mut self) {
        self.game_object.set_position(self.physics.position);
    }
}
