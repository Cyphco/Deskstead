// External crate imports
use raylib::prelude::*;
use super::base::GameObject;
use std::ops::Range;
use std::time::{Duration, Instant};

/// Represents a single particle in the particle system
/// Each particle has position, movement, appearance, and lifetime properties
pub struct Particle {
    position: Vector2,
    velocity: Vector2,
    acceleration: Vector2,
    rotation: f32,
    rotation_speed: f32,
    scale: f32,
    scale_speed: f32,
    alpha: f32,
    alpha_speed: f32,
    lifetime: Duration,
    birth_time: Instant,
    color: Color,
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
        alpha: f32,
        alpha_speed: f32,
        lifetime: Duration,
        color: Color,
    ) -> Self {
        Self {
            position,
            velocity,
            acceleration,
            rotation,
            rotation_speed,
            scale,
            scale_speed,
            alpha,
            alpha_speed,
            lifetime,
            birth_time: Instant::now(),
            color,
        }
    }

    /// Updates the particle's state and returns whether it's still alive
    #[inline(always)]
    pub fn update(&mut self, dt: f32) {
        self.velocity += self.acceleration * dt;
        self.position += self.velocity * dt;
        self.rotation += self.rotation_speed * dt;
        self.scale += self.scale_speed * dt;
        self.alpha += self.alpha_speed * dt;
        self.alpha = self.alpha.clamp(0.0, 1.0);
    }

    /// Draws the particle
    #[inline(always)]
    pub fn draw(&self, d: &mut RaylibDrawHandle, texture: Option<&Texture2D>) {
        let mut color = self.color;
        color.a = (self.alpha * 255.0) as u8;

        if let Some(tex) = texture {
            d.draw_texture_pro(
                tex,
                Rectangle::new(0.0, 0.0, tex.width as f32, tex.height as f32),
                Rectangle::new(
                    self.position.x,
                    self.position.y,
                    tex.width as f32 * self.scale,
                    tex.height as f32 * self.scale,
                ),
                Vector2::new(tex.width as f32 * self.scale / 2.0, tex.height as f32 * self.scale / 2.0),
                self.rotation,
                color,
            );
        } else {
            let size = 10.0 * self.scale;
            d.draw_circle_v(self.position, size, color);
        }
    }

    /// Returns whether the particle is still active
    #[inline(always)]
    pub fn is_alive(&self) -> bool {
        self.birth_time.elapsed() < self.lifetime && self.alpha > 0.0
    }
}

/// Configuration settings for the particle system
/// Controls various aspects like particle count, lifetime, movement, and appearance
pub struct ParticleSettings {
    pub emission_rate: f32,            // Particles per second
    pub lifetime_range: Range<u64>,    // Milliseconds
    pub speed_range: Range<f32>,       // Pixels per second
    pub scale_range: Range<f32>,       // Range for particle size scaling
    pub rotation_speed_range: Range<f32>, // Radians per second
    pub scale_speed: f32,              // Scale change per second
    pub alpha_speed: f32,              // Alpha change per second
    pub gravity: Vector2,              // Acceleration due to gravity
    pub color: Color,                  // Base color for particles
}

impl Default for ParticleSettings {
    fn default() -> Self {
        Self {
            emission_rate: 10.0,
            lifetime_range: 1000..2000,
            speed_range: 50.0..100.0,
            scale_range: 0.5..1.5,
            rotation_speed_range: -1.0..1.0,
            scale_speed: -0.5,
            alpha_speed: -0.5,
            gravity: Vector2::new(0.0, 98.1),
            color: Color::WHITE,
        }
    }
}

/// Main particle system that manages creation, updates, and rendering of particles
pub struct ParticleSystem {
    position: Vector2,
    particles: Vec<Particle>,
    texture: Option<Texture2D>,
    max_particles: usize,
    settings: ParticleSettings,
    emission_area: Vector2,
}

impl ParticleSystem {
    /// Creates a new particle with randomized properties within configured ranges
    #[inline(always)]
    fn create_particle(&self) -> Particle {
        let angle = rand::random::<f32>() * std::f32::consts::PI * 2.0;
        let speed = rand::random::<f32>() * (self.settings.speed_range.end - self.settings.speed_range.start) + self.settings.speed_range.start;
        
        let velocity = Vector2::new(
            angle.cos() * speed,
            angle.sin() * speed,
        );

        let lifetime = rand::random::<u64>() % (self.settings.lifetime_range.end - self.settings.lifetime_range.start) + self.settings.lifetime_range.start;
        
        Particle::new(
            self.position,
            velocity,
            self.settings.gravity,
            0.0,
            rand::random::<f32>() * self.settings.rotation_speed_range.end,
            rand::random::<f32>() * (self.settings.scale_range.end - self.settings.scale_range.start) + self.settings.scale_range.start,
            self.settings.scale_speed,
            1.0,
            self.settings.alpha_speed,
            Duration::from_millis(lifetime),
            self.settings.color,
        )
    }

    /// Updates all particles and emits new ones based on emission rate
    fn update_particles(&mut self, dt: f32) {
        // Update existing particles
        self.particles.retain_mut(|p| {
            p.update(dt);
            p.is_alive()
        });

        // Generate new particles
        let new_particles_count = (self.settings.emission_rate * dt) as usize;
        for _ in 0..new_particles_count {
            if self.particles.len() >= self.max_particles {
                break;
            }

            let particle = self.create_particle();
            self.particles.push(particle);
        }
    }

    /// Renders a batch of particles using either a texture or circles
    #[inline(always)]
    fn draw_batch(&self, d: &mut RaylibDrawHandle, batch: &[Particle]) {
        for particle in batch {
            particle.draw(d, self.texture.as_ref());
        }
    }
}

impl GameObject for ParticleSystem {
    /// Renders all particles in the system
    fn draw(&self, d: &mut RaylibDrawHandle) {
        let batch_size = 100;
        let mut i = 0;
        while i < self.particles.len() {
            let end = (i + batch_size).min(self.particles.len());
            let batch = &self.particles[i..end];
            self.draw_batch(d, batch);
            i += batch_size;
        }
    }

    /// Updates the particle system state
    fn update(&mut self, dt: f32) {
        self.update_particles(dt);
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
        0.0  // Particle systems don't rotate as a whole
    }

    /// Sets the rotation of the particle system
    fn set_rotation(&mut self, _rotation: f32) {
        // Particle systems don't rotate as a whole
    }

    /// Gets the size of the particle system
    fn get_size(&self) -> Vector2 {
        self.emission_area
    }
}

/// Builder pattern implementation for configuring a ParticleSystem
pub struct ParticleSystemBuilder {
    position: Vector2,
    texture: Option<Texture2D>,
    settings: ParticleSettings,
    max_particles: usize,
    emission_area: Vector2,
}

impl ParticleSystemBuilder {
    /// Creates a new ParticleSystemBuilder for configuring a particle system
    pub fn new(position: Vector2) -> Self {
        Self {
            position,
            texture: None,
            settings: ParticleSettings::default(),
            max_particles: 1000,
            emission_area: Vector2::new(100.0, 100.0),
        }
    }

    /// Sets the texture used for rendering particles
    pub fn texture(mut self, texture: Texture2D) -> Self {
        self.texture = Some(texture);
        self
    }

    /// Sets the particle system settings
    pub fn settings(mut self, settings: ParticleSettings) -> Self {
        self.settings = settings;
        self
    }

    /// Sets the maximum number of particles
    pub fn max_particles(mut self, max: usize) -> Self {
        self.max_particles = max;
        self
    }

    /// Sets the emission area size
    pub fn emission_area(mut self, area: Vector2) -> Self {
        self.emission_area = area;
        self
    }

    /// Builds and returns a configured ParticleSystem
    pub fn build(self) -> ParticleSystem {
        ParticleSystem {
            position: self.position,
            particles: Vec::with_capacity(self.max_particles),
            texture: self.texture,
            max_particles: self.max_particles,
            settings: self.settings,
            emission_area: self.emission_area,
        }
    }
}
