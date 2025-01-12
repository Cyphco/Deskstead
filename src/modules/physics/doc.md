# Physics Module Documentation

## Overview
The physics module implements a 2D physics simulation system for the Deskstead engine. It provides realistic movement, collisions, and force interactions between game objects.

## Core Components

### PhysicsComponent (component.rs)
A trait that defines the interface for physics-enabled objects:
- Access to physics body properties
- State synchronization between physics and visual representation
- Integration with the main physics world

### PhysicsBody (body.rs)
The core physics simulation unit:
- Position and velocity management
- Mass and inertia calculations
- Collision shape definition
- Force application points
- Fixed/dynamic state control

### Forces (forces.rs)
Handles various types of forces in the physics simulation:
- Gravity
- Spring forces
- Drag and friction
- Custom force implementations

### PhysicsWorld (world.rs)
The main physics simulation container:
- Object management
- Collision detection and resolution
- Force application
- Time step management
- Spatial partitioning for efficient collision checks

## Usage Examples

### Creating a Physics Body
```rust
let body = PhysicsBody::new(
    Vector2::new(0.0, 0.0), // position
    1.0                     // mass
);
```

### Implementing Physics Component
```rust
impl PhysicsComponent for MyObject {
    fn get_physics_body(&self) -> &PhysicsBody {
        &self.physics
    }
    
    fn get_physics_body_mut(&mut self) -> &mut PhysicsBody {
        &mut self.physics
    }
    
    fn sync_with_physics(&mut self) {
        // Update object state based on physics
    }
}
```

### Physics World Usage
```rust
let mut world = PhysicsWorld::new();
world.add_body(physics_object);
world.update(dt);
```

## Integration Points
- Works seamlessly with the objects module through PhysicsObject wrapper
- Can be extended with custom force implementations
- Supports both dynamic and static bodies
- Efficient broad-phase collision detection
