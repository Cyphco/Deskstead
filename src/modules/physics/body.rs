use raylib::prelude::*;

/// Represents a physics body with position, velocity, and other physical properties
#[derive(Clone, Debug)]
pub struct PhysicsBody {
    pub position: Vector2,
    pub velocity: Vector2,
    pub acceleration: Vector2,
    pub size: Vector2,
    pub mass: f32,
    pub restitution: f32,  // Bounciness factor (0 to 1)
    pub friction: f32,     // Friction coefficient
    pub fixed: bool,       // If true, object doesn't move
    accumulated_force: Vector2,
}

impl Default for PhysicsBody {
    fn default() -> Self {
        Self {
            position: Vector2::zero(),
            velocity: Vector2::zero(),
            acceleration: Vector2::zero(),
            size: Vector2::new(50.0, 50.0),
            mass: 1.0,
            restitution: 0.5,
            friction: 0.1,
            fixed: false,
            accumulated_force: Vector2::zero(),
        }
    }
}

impl PhysicsBody {
    /// Creates a new physics body at the specified position
    pub fn new(position: Vector2, mass: f32) -> Self {
        Self {
            position,
            mass,
            ..Default::default()
        }
    }

    /// Applies a force to the body
    pub fn apply_force(&mut self, force: Vector2) {
        if !self.fixed {
            self.accumulated_force += force;
        }
    }

    /// Applies an instantaneous impulse to the body
    pub fn apply_impulse(&mut self, impulse: Vector2) {
        if !self.fixed {
            self.velocity += impulse / self.mass;
        }
    }

    /// Updates the physics body's state
    pub fn update(&mut self, dt: f32) {
        if self.fixed {
            self.velocity = Vector2::zero();
            self.acceleration = Vector2::zero();
            self.accumulated_force = Vector2::zero();
            return;
        }

        // Calculate acceleration from accumulated forces
        self.acceleration = self.accumulated_force / self.mass;

        // Update velocity using acceleration
        self.velocity += self.acceleration * dt;

        // Apply velocity limits to prevent extreme speeds
        let max_speed = 2000.0;
        let speed = self.velocity.length();
        if speed > max_speed {
            self.velocity = self.velocity.normalized() * max_speed;
        }

        // Update position using velocity
        self.position += self.velocity * dt;

        // Reset accumulated forces
        self.accumulated_force = Vector2::zero();
    }

    /// Sets the size of the physics body
    pub fn set_size(&mut self, size: Vector2) {
        self.size = size;
    }

    /// Sets whether the body is fixed (static) or not
    pub fn set_fixed(&mut self, fixed: bool) {
        self.fixed = fixed;
        if fixed {
            self.velocity = Vector2::zero();
            self.acceleration = Vector2::zero();
            self.accumulated_force = Vector2::zero();
        }
    }
}
