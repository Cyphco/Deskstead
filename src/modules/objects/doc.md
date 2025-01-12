# Instructions for the object system

## Particle.rs
The Particle System is designed to create and manage particles in a graphical application. This guide provides detailed usage instructions for the `particle.rs` module.
<details>
<summary>Click to expand</summary>

## Particle System Configuration

### Parameters
1. **Texture**: The texture used for the particles.
   - **Type**: `Texture`
   - **Usage**: `.texture(your_texture)`

2. **Scale**: The scale of the particles.
   - **Type**: `f32`
   - **Usage**: `.scale(0.1)` (default is `1.0`)

3. **Emission Rate**: The rate at which particles are emitted.
   - **Type**: `f32`
   - **Usage**: `.emission_rate(1.0)`

4. **Position**: The initial position of the particles.
   - **Type**: `Vector2`
   - **Usage**: `.position(Vector2::new(x, y))`

5. **Gravity**: The gravity vector affecting the particles.
   - **Type**: `Vector2`
   - **Usage**: `.gravity(Vector2::new(0.0, -9.81))`

6. **Angle Range**: The range of angles for particle emission.
   - **Type**: `Range<f32>`
   - **Usage**: `.angle_range(0.0..45.0)`

7. **Rotation Speed**: The speed at which particles rotate.
   - **Type**: `Range<f32>`
   - **Usage**: `.rotation_speed(0.0..1.0)`

8. **Initial Speed**: The initial speed of the particles.
   - **Type**: `Range<f32>`
   - **Usage**: `.initial_speed(0.0..1.0)`

### Example Usage
```rust
let mut particle_system = ParticleSystem::new()
    .texture(your_texture)
    .scale(0.1)
    .emission_rate(1.0)
    .position(Vector2::new(cursor_vec.x, cursor_vec.y))
    .gravity(Vector2::new(0.0, 0.0))
    .angle_range(0.0..45.0)
    .rotation_speed(0.0..1.0)
    .initial_speed(0.0..1.0)
    .build();
```



### Emitting Particles
The `emit_particle` function is responsible for creating new particles based on the configured settings. It samples the angle, speed, and rotation speed from the defined ranges and creates a new `Particle` object.

### Important Notes
- Ensure that the ranges for angle, rotation speed, and initial speed are valid to prevent runtime errors.
- The particle system can be updated and drawn in your main loop to visualize the particles.

## Conclusion
This guide provides a comprehensive overview of how to use the particle system in your application. For further customization, refer to the source code and adjust parameters as needed.
</details>