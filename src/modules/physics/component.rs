use crate::modules::physics::body::PhysicsBody;

/// Trait for objects that can have physics behavior
pub trait PhysicsComponent {
    /// Get a reference to the physics body
    fn get_physics_body(&self) -> &PhysicsBody;
    
    /// Get a mutable reference to the physics body
    fn get_physics_body_mut(&mut self) -> &mut PhysicsBody;
    
    /// Called after physics update to sync object state with physics
    fn sync_with_physics(&mut self);
}
