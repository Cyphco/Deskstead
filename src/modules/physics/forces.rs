use raylib::prelude::*;
use super::body::PhysicsBody;

/// Applies gravitational force to a physics body
pub fn apply_gravity(body: &mut PhysicsBody, gravity: Vector2) {
    if !body.fixed {
        let force = gravity * body.mass;
        body.apply_force(force);
    }
}

/// Applies drag force to a physics body
pub fn apply_drag(body: &mut PhysicsBody, coefficient: f32) {
    if !body.fixed {
        let force = -body.velocity * coefficient;
        body.apply_force(force);
    }
}

/// Applies a spring force between two bodies
pub fn apply_spring_force(body1: &mut PhysicsBody, body2: &PhysicsBody, rest_length: f32, stiffness: f32, damping: f32) {
    if !body1.fixed {
        let displacement = body2.position - body1.position;
        let distance = displacement.length();
        
        if distance > 0.0 {
            // Calculate spring force
            let spring_force = displacement.normalized() * (distance - rest_length) * stiffness;
            
            // Calculate damping force
            let relative_velocity = body2.velocity - body1.velocity;
            let damping_force = relative_velocity * damping;
            
            // Apply combined force
            body1.apply_force(spring_force + damping_force);
        }
    }
}

/// Applies a repulsion force between two bodies
pub fn apply_repulsion(body1: &mut PhysicsBody, body2: &PhysicsBody, min_distance: f32, strength: f32) {
    if !body1.fixed {
        let displacement = body1.position - body2.position;
        let distance = displacement.length();
        
        if distance > 0.0 && distance < min_distance {
            let force = displacement.normalized() * strength * (1.0 - distance / min_distance);
            body1.apply_force(force);
        }
    }
}

/// Applies friction force to a body
pub fn apply_friction(body: &mut PhysicsBody, normal_force: f32) {
    if !body.fixed && body.velocity.length_sqr() > 0.0 {
        let friction_force = -body.velocity.normalized() * normal_force * body.friction;
        body.apply_force(friction_force);
    }
}
