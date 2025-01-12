use raylib::prelude::*;
use super::base::GameObject;
use std::time::{Duration, Instant};

pub struct AnimatedObject {
    frames: Vec<Texture2D>,
    current_frame: usize,
    frame_duration: Duration,
    last_frame_time: Instant,
    position: Vector2,
    rotation: f32,
    scale: f32,
    is_playing: bool,
    loop_animation: bool,
}

impl AnimatedObject {
    pub fn new(
        window: &mut RaylibHandle,
        thread: &RaylibThread,
        frame_paths: Vec<&str>,
        frame_duration_ms: u64,
        x: f32,
        y: f32,
        rotation: f32,
        scale: f32,
    ) -> Self {
        let frames = frame_paths
            .iter()
            .map(|path| window.load_texture(thread, path).expect("Couldn't load texture"))
            .collect();

        Self {
            frames,
            current_frame: 0,
            frame_duration: Duration::from_millis(frame_duration_ms),
            last_frame_time: Instant::now(),
            position: Vector2::new(x, y),
            rotation,
            scale,
            is_playing: true,
            loop_animation: true,
        }
    }

    pub fn play(&mut self) {
        self.is_playing = true;
    }

    pub fn pause(&mut self) {
        self.is_playing = false;
    }

    pub fn set_looping(&mut self, loop_animation: bool) {
        self.loop_animation = loop_animation;
    }

    pub fn reset(&mut self) {
        self.current_frame = 0;
        self.last_frame_time = Instant::now();
    }

    fn update_animation(&mut self) {
        if !self.is_playing || self.frames.is_empty() {
            return;
        }

        let now = Instant::now();
        if now.duration_since(self.last_frame_time) >= self.frame_duration {
            self.current_frame = (self.current_frame + 1) % self.frames.len();
            if !self.loop_animation && self.current_frame == 0 {
                self.is_playing = false;
                self.current_frame = self.frames.len() - 1;
            }
            self.last_frame_time = now;
        }
    }
}

impl GameObject for AnimatedObject {
    fn draw(&self, draw_handle: &mut RaylibDrawHandle) {
        if let Some(current_texture) = self.frames.get(self.current_frame) {
            draw_handle.draw_texture_ex(
                current_texture,
                self.position,
                self.rotation,
                self.scale,
                Color::WHITE,
            );
        }
    }

    fn get_position(&self) -> Vector2 {
        self.position
    }

    fn set_position(&mut self, position: Vector2) {
        self.position = position;
    }

    fn get_rotation(&self) -> f32 {
        self.rotation
    }

    fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;
    }

    fn get_scale(&self) -> f32 {
        self.scale
    }

    fn set_scale(&mut self, scale: f32) {
        self.scale = scale;
    }

    fn update(&mut self) {
        self.update_animation();
    }
}
