use raylib::prelude::*;
use super::{
    physics_object::PhysicsObject,
    textured::TexturedObject,
};
use crate::modules::physics::component::PhysicsComponent;

pub struct PhysicsObjectBuilder {
    texture: Option<Texture2D>,
    position: Vector2,
    size: Vector2,
    mass: f32,
    fixed: bool,
    restitution: f32,
    friction: f32,
    rotation: f32,
    initial_velocity: Vector2,
}

impl Default for PhysicsObjectBuilder {
    fn default() -> Self {
        Self {
            texture: None,
            position: Vector2::zero(),
            size: Vector2::new(50.0, 50.0),
            mass: 1.0,
            fixed: false,
            restitution: 0.5,
            friction: 0.1,
            rotation: 0.0,
            initial_velocity: Vector2::zero(),
        }
    }
}

impl PhysicsObjectBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_texture(mut self, texture: Texture2D) -> Self {
        self.texture = Some(texture);
        self
    }

    pub fn at_position(mut self, position: Vector2) -> Self {
        self.position = position;
        self
    }

    pub fn with_size(mut self, size: Vector2) -> Self {
        self.size = size;
        self
    }

    pub fn with_mass(mut self, mass: f32) -> Self {
        self.mass = mass;
        self
    }

    pub fn fixed(mut self) -> Self {
        self.fixed = true;
        self
    }

    pub fn with_restitution(mut self, restitution: f32) -> Self {
        self.restitution = restitution;
        self
    }

    pub fn with_friction(mut self, friction: f32) -> Self {
        self.friction = friction;
        self
    }

    pub fn with_rotation(mut self, rotation: f32) -> Self {
        self.rotation = rotation;
        self
    }

    pub fn with_initial_velocity(mut self, velocity: Vector2) -> Self {
        self.initial_velocity = velocity;
        self
    }

    pub fn build(self) -> PhysicsObject<TexturedObject> {
        let texture = self.texture.expect("Texture must be provided before building");
        
        let textured = TexturedObject::new(
            texture,
            self.position,
            self.size,
            self.rotation,
        );

        let mut physics_obj = PhysicsObject::new(
            textured,
            self.position,
            self.size,
            self.mass,
            self.fixed,
        );

        // Set additional physics properties
        let physics_body = physics_obj.get_physics_body_mut();
        physics_body.restitution = self.restitution;
        physics_body.friction = self.friction;
        physics_body.velocity = self.initial_velocity;

        physics_obj
    }
}
