// External crate imports
use raylib::prelude::*;
use super::base::GameObject;
use std::time::{Duration, Instant};
use rand::Rng;
use std::ops::Range;

/// Represents a single particle in the particle system
/// Each particle has position, movement, appearance, and lifetime properties
#[derive(Clone)]
pub struct Particle {
    position: Vector2,
    velocity: Vector2,
    acceleration: Vector2,
    rotation: f32,
    rotation_speed: f32,
    scale: f32,
    scale_speed: f32,
    color: Color,
    alpha_speed: f32,
    birth_time: Instant,
    lifetime: Duration,
    is_alive: bool,
}

/// Configuration settings for the particle system
/// Controls various aspects like particle count, lifetime, movement, and appearance
pub struct ParticleSettings {
    pub max_particles: usize,         // Maximum number of particles allowed in the system
    pub particle_lifetime: Range<u64>, // Range for particle lifetime in milliseconds
    pub initial_speed: Range<f32>,    // Range for initial particle velocity
    pub scale: Range<f32>,           // Range for particle size scaling
    pub rotation_speed: Range<f32>,   // Range for particle rotation speed
    pub initial_rotation: Range<f32>, // Range for initial particle rotation
    pub alpha_speed: f32,            // Rate of change for particle transparency
    pub scale_speed: f32,            // Rate of change for particle size
    pub gravity: Vector2,            // Gravitational force applied to particles
    pub batch_size: usize,           // Number of particles to process in one batch
    pub angle_range: Range<f32>,     // Range of angles (in radians) for particle emission
}

/// Default implementation for ParticleSettings
impl Default for ParticleSettings {
    fn default() -> Self {
        Self {
            max_particles: 1_000_000,
            particle_lifetime: 1000..3000,
            initial_speed: 1.0..5.0,
            scale: 0.5..1.5,
            rotation_speed: 0.0..0.1,  // Default to no rotation
            initial_rotation: 0.0..0.1, // Default to no initial rotation
            alpha_speed: -1.0,
            scale_speed: -0.01,
            gravity: Vector2::new(0.0, 0.0),
            batch_size: 10000,
            angle_range: 0.0..std::f32::consts::PI * 2.0,
        }
    }
}

impl Particle {
    /// Creates a new particle with the specified properties
    #[inline(always)]
    pub fn new(
        position: Vector2,
        velocity: Vector2,
        acceleration: Vector2,
        rotation: f32,
        rotation_speed: f32,
        scale: f32,
        scale_speed: f32,
        color: Color,
        alpha_speed: f32,
        lifetime_ms: u64,
    ) -> Self {
        Self {
            position,
            velocity,
            acceleration,
            rotation,
            rotation_speed,
            scale,
            scale_speed,
            color,
            alpha_speed,
            lifetime: Duration::from_millis(lifetime_ms),
            birth_time: Instant::now(),
            is_alive: true,
        }
    }

    /// Returns whether the particle is still active
    #[inline(always)]
    pub fn is_alive(&self) -> bool {
        self.is_alive
    }

    /// Updates the particle's state and returns whether it's still alive
    #[inline(always)]
    pub fn update(&mut self) -> bool {
        if !self.is_alive {
            return false;
        }

        self.velocity = self.velocity + self.acceleration;
        self.position = self.position + self.velocity;
        self.rotation += self.rotation_speed;
        self.scale += self.scale_speed;
        self.color.a = ((self.color.a as f32) + self.alpha_speed) as u8;
        
        let age = Instant::now().duration_since(self.birth_time);
        self.is_alive = age < self.lifetime && self.color.a > 0;
        
        self.is_alive
    }
}

/// Main particle system that manages creation, updates, and rendering of particles
pub struct ParticleSystem {
    particles: Vec<Particle>,
    position: Vector2,
    rotation: f32,
    scale: f32,
    emission_rate: f32,
    last_emission: Instant,
    texture: Option<Texture2D>,
    tint: Option<Color>,
    settings: ParticleSettings,
    rng: rand::rngs::ThreadRng,
    last_update: Instant,
    shader: Option<Shader>,
}

/// Default implementation for ParticleSystem
impl Default for ParticleSystem {
    fn default() -> Self {
        Self {
            particles: Vec::with_capacity(ParticleSettings::default().max_particles),
            position: Vector2::zero(),
            rotation: 0.0,
            scale: 1.0,
            emission_rate: 1000.0,
            last_emission: Instant::now(),
            texture: None,
            tint: None,
            settings: ParticleSettings::default(),
            rng: rand::thread_rng(),
            last_update: Instant::now(),
            shader: None,
        }
    }
}

impl ParticleSystem {
    /// Creates a new ParticleSystemBuilder for configuring a particle system
    pub fn new() -> ParticleSystemBuilder {
        ParticleSystemBuilder::default()
    }

    /// Returns the maximum number of particles allowed in the system
    #[inline(always)]
    pub fn get_max_particles(&self) -> usize {
        self.settings.max_particles
    }

    /// Returns the current number of active particles
    #[inline(always)]
    pub fn get_particle_count(&self) -> usize {
        self.particles.len()
    }

    /// Creates a new particle with randomized properties within configured ranges
    #[inline(always)]
    fn emit_particle(&mut self) {
        if self.particles.len() >= self.settings.max_particles {
            return;
        }

        let angle = self.rng.gen_range(self.settings.angle_range.start..self.settings.angle_range.end);
        let speed = self.rng.gen_range(self.settings.initial_speed.start..self.settings.initial_speed.end);
        let velocity = Vector2::new(angle.cos() * speed, angle.sin() * speed);
        let initial_rotation = self.rng.gen_range(self.settings.initial_rotation.start..self.settings.initial_rotation.end);
        let rotation_speed = self.rng.gen_range(self.settings.rotation_speed.start..self.settings.rotation_speed.end);

        
        let particle = Particle::new(
            self.position,
            velocity,
            self.settings.gravity,
            initial_rotation,
            rotation_speed,
            self.rng.gen_range(self.settings.scale.start..self.settings.scale.end) * self.scale,
            self.settings.scale_speed,
            self.tint.unwrap_or(Color::WHITE),
            self.settings.alpha_speed,
            self.rng.gen_range(self.settings.particle_lifetime.start..self.settings.particle_lifetime.end),
        );
        
        self.particles.push(particle);
    }

    /// Updates all particles and emits new ones based on emission rate
    fn update_particles(&mut self) {
        let now = Instant::now();
        let time_since_emission = now.duration_since(self.last_emission).as_secs_f32();
        let particles_to_emit = (time_since_emission * self.emission_rate) as usize;

        // Emit new particles
        for _ in 0..particles_to_emit {
            self.emit_particle();
        }
        
        if particles_to_emit > 0 {
            self.last_emission = now;
        }

        // Update existing particles
        let mut i = 0;
        while i < self.particles.len() {
            if !self.particles[i].update() {
                // Remove dead particle
                self.particles.swap_remove(i);
            } else {
                i += 1;
            }
        }
    }

    /// Renders a batch of particles using either a texture or circles
    #[inline(always)]
    fn draw_batch(&self, draw_handle: &mut RaylibDrawHandle, batch: &[Particle]) {
        if let Some(shader) = &self.shader {
            let mut shader_mode = draw_handle.begin_shader_mode(shader);
            match &self.texture {
                Some(texture) => {
                    for particle in batch {
                        shader_mode.draw_texture_ex(
                            texture,
                            particle.position,
                            particle.rotation,
                            particle.scale,
                            particle.color,
                        );
                    }
                }
                None => {
                    for particle in batch {
                        shader_mode.draw_circle_v(
                            particle.position,
                            5.0 * particle.scale,
                            particle.color,
                        );
                    }
                }
            }
        } else {
            match &self.texture {
                Some(texture) => {
                    for particle in batch {
                        draw_handle.draw_texture_ex(
                            texture,
                            particle.position,
                            particle.rotation,
                            particle.scale,
                            particle.color,
                        );
                    }
                }
                None => {
                    for particle in batch {
                        draw_handle.draw_circle_v(
                            particle.position,
                            5.0 * particle.scale,
                            particle.color,
                        );
                    }
                }
            }
        }
    }
}

/// Builder pattern implementation for configuring a ParticleSystem
pub struct ParticleSystemBuilder {
    position: Vector2,
    rotation: f32,
    scale: f32,
    emission_rate: f32,
    texture: Option<Texture2D>,
    tint: Option<Color>,
    settings: ParticleSettings,
    shader: Option<Shader>,
}

/// Default implementation for ParticleSystemBuilder
impl Default for ParticleSystemBuilder {
    fn default() -> Self {
        Self {
            position: Vector2::zero(),
            rotation: 0.0,
            scale: 1.0,
            emission_rate: 1000.0,
            texture: None,
            tint: None,
            settings: ParticleSettings::default(),
            shader: None,
        }
    }
}

/// Builder methods for ParticleSystemBuilder
impl ParticleSystemBuilder {
    /// Sets the position of the particle system
    pub fn position(mut self, position: Vector2) -> Self {
        self.position = position;
        self
    }

    /// Sets the rotation of the particle system
    pub fn rotation(mut self, rotation: f32) -> Self {
        self.rotation = rotation;
        self
    }

    /// Sets the scale of the particle system
    pub fn scale(mut self, scale: f32) -> Self {
        self.scale = scale;
        self
    }

    /// Sets the emission rate (particles per second)
    pub fn emission_rate(mut self, rate: f32) -> Self {
        self.emission_rate = rate;
        self
    }

    /// Sets the texture used for rendering particles
    pub fn texture(mut self, texture: Texture2D) -> Self {
        self.texture = Some(texture);
        self
    }

    /// Sets the base color tint for particles
    pub fn tint(mut self, color: Color) -> Self {
        self.tint = Some(color);
        self
    }

    /// Sets the particle system settings
    pub fn settings(mut self, settings: ParticleSettings) -> Self {
        self.settings = settings;
        self
    }

    /// Sets the maximum number of particles
    pub fn max_particles(mut self, max: usize) -> Self {
        self.settings.max_particles = max;
        self
    }

    /// Sets the lifetime range for particles
    pub fn particle_lifetime(mut self, range: Range<u64>) -> Self {
        self.settings.particle_lifetime = range;
        self
    }

    /// Sets the initial speed range for particles
    pub fn initial_speed(mut self, range: Range<f32>) -> Self {
        // Allow a range of 0 for no speed
        self.settings.initial_speed = range; 
        self
    }

    /// Sets the scale range for particles
    pub fn particle_scale(mut self, range: Range<f32>) -> Self {
        self.settings.scale = range;
        self
    }

    /// Sets the rotation speed range for particles
    pub fn rotation_speed(mut self, range: Range<f32>) -> Self {  
        self.settings.rotation_speed = range; 
        self
    }

    /// Sets the initial rotation range for particles (in radians)
    pub fn initial_rotation(mut self, range: Range<f32>) -> Self {
        self.settings.initial_rotation = range;
        self
    }
    

    /// Sets the alpha (transparency) change rate
    pub fn alpha_speed(mut self, speed: f32) -> Self {
        self.settings.alpha_speed = speed;
        self
    }

    /// Sets the scale change rate
    pub fn scale_speed(mut self, speed: f32) -> Self {
        self.settings.scale_speed = speed;
        self
    }

    /// Sets the gravitational force
    pub fn gravity(mut self, gravity: Vector2) -> Self {
        self.settings.gravity = gravity;
        self
    }

    /// Sets the angle range for particle emission (in radians)
    pub fn angle_range(mut self, range: Range<f32>) -> Self {
        // Allow a range of 0 for no spread
        self.settings.angle_range = range; // Directly assign the range
        self
    }

    /// Sets the shader for particle rendering
    pub fn shader(mut self, shader: Shader) -> Self {
        self.shader = Some(shader);
        self
    }

    /// Builds and returns a configured ParticleSystem
    pub fn build(self) -> ParticleSystem {
        ParticleSystem {
            particles: Vec::with_capacity(self.settings.max_particles),
            position: self.position,
            rotation: self.rotation,
            scale: self.scale,
            emission_rate: self.emission_rate,
            last_emission: Instant::now(),
            texture: self.texture,
            tint: self.tint,
            settings: self.settings,
            rng: rand::thread_rng(),
            last_update: Instant::now(),
            shader: self.shader,
        }
    }
}

/// GameObject trait implementation for ParticleSystem
impl GameObject for ParticleSystem {
    /// Renders all particles in the system
    fn draw(&self, draw_handle: &mut RaylibDrawHandle) {
        let batch_size = self.settings.batch_size;
        let mut i = 0;
        while i < self.particles.len() {
            let end = (i + batch_size).min(self.particles.len());
            let batch = &self.particles[i..end];
            self.draw_batch(draw_handle, batch);
            i += batch_size;
        }
    }

    /// Updates the particle system state
    fn update(&mut self) {
        self.update_particles();
    }

    /// Gets the current position of the particle system
    fn get_position(&self) -> Vector2 {
        self.position
    }

    /// Sets the position of the particle system
    fn set_position(&mut self, position: Vector2) {
        self.position = position;
    }

    /// Gets the current rotation of the particle system
    fn get_rotation(&self) -> f32 {
        self.rotation
    }

    /// Sets the rotation of the particle system
    fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;
    }

    /// Gets the current scale of the particle system
    fn get_scale(&self) -> f32 {
        self.scale
    }

    /// Sets the scale of the particle system
    fn set_scale(&mut self, scale: f32) {
        self.scale = scale;
    }
}
