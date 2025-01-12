use raylib::prelude::*;
use super::base::GameObject;

pub struct AnimatedObject {
    texture: &'static Texture2D,
    position: Vector2,
    rotation: f32,
    size: Vector2,
    frame_width: i32,
    frame_height: i32,
    current_frame: i32,
    frames_counter: i32,
    frames_speed: i32,
    max_frames: i32,
    playing: bool,
    looping: bool,
}

impl AnimatedObject {
    pub fn new(
        texture: &'static Texture2D,
        position: Vector2,
        size: Vector2,
        frame_width: i32,
        frame_height: i32,
        frames_speed: i32,
        max_frames: i32,
    ) -> Self {
        Self {
            texture,
            position,
            rotation: 0.0,
            size,
            frame_width,
            frame_height,
            current_frame: 0,
            frames_counter: 0,
            frames_speed,
            max_frames,
            playing: false,
            looping: false,
        }
    }

    pub fn play(&mut self) {
        self.playing = true;
    }

    pub fn pause(&mut self) {
        self.playing = false;
    }

    pub fn set_looping(&mut self, loop_animation: bool) {
        self.looping = loop_animation;
    }

    pub fn reset(&mut self) {
        self.current_frame = 0;
        self.frames_counter = 0;
    }
}

impl GameObject for AnimatedObject {
    fn update(&mut self, dt: f32) {
        if !self.playing {
            return;
        }

        self.frames_counter += 1;

        if self.frames_counter >= self.frames_speed {
            self.frames_counter = 0;
            self.current_frame += 1;

            if self.current_frame >= self.max_frames {
                if self.looping {
                    self.current_frame = 0;
                } else {
                    self.current_frame = self.max_frames - 1;
                    self.playing = false;
                }
            }
        }
    }

    fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_texture_pro(
            self.texture,
            Rectangle::new(
                self.current_frame as f32 * self.frame_width as f32,
                0.0,
                self.frame_width as f32,
                self.frame_height as f32,
            ),
            Rectangle::new(
                self.position.x,
                self.position.y,
                self.size.x,
                self.size.y,
            ),
            Vector2::new(self.size.x / 2.0, self.size.y / 2.0),
            self.rotation,
            Color::WHITE,
        );
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

    fn get_size(&self) -> Vector2 {
        self.size
    }
}
