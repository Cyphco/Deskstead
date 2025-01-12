# Objects Module Documentation

## Overview
The objects module provides a flexible and extensible system for creating and managing game objects in the Deskstead engine. It includes support for basic objects, textured objects, animated objects, particles, and physics-enabled objects.

## Core Components

### GameObject (base.rs)
The foundation trait that all game objects must implement. Provides basic functionality for:
- Update logic
- Drawing
- Position management
- Rotation handling

### TexturedObject (textured.rs)
Extends the base GameObject with texture rendering capabilities:
- Texture loading and management
- Scale and size control
- Texture drawing with position and rotation

### AnimatedObject (animated.rs)
Provides support for sprite-based animations:
- Frame-based animation system
- Multiple animation states
- Animation speed control
- Automatic frame updates

### ParticleSystem (particle.rs)
A comprehensive particle system for special effects:
- Particle emission control
- Lifetime management
- Movement patterns
- Color and size transitions

### PhysicsObject (physics_object.rs)
A wrapper that adds physics capabilities to any GameObject:
- Integration with the physics system
- Collision detection and response
- Mass and force handling
- Position synchronization with physics simulation

## Usage Examples

### Creating a Basic Object
```rust
impl GameObject for MyObject {
    fn update(&mut self, dt: f32) {
        // Update logic
    }
    
    fn draw(&self, draw_handle: &mut RaylibDrawHandle) {
        // Drawing logic
    }
}
```

### Adding Physics to an Object
```rust
let game_object = MyObject::new();
let physics_object = PhysicsObject::new(
    game_object,
    Vector2::new(0.0, 0.0),  // position
    Vector2::new(32.0, 32.0), // size
    1.0,                      // mass
    false                     // fixed
);
```

## Integration Points
- All objects can be wrapped with PhysicsObject for physics simulation
- Particle systems can be attached to any GameObject
- AnimatedObject can be combined with physics for dynamic game entities