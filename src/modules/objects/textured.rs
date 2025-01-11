use raylib::prelude::*;
use super::base::GameObject;

pub struct TexturedObject {
    texture: Texture2D,
    position: Vector2,
    rotation: f32,
    scale: f32,
}

impl TexturedObject {
    pub fn new(window: &mut RaylibHandle, thread: &RaylibThread, texture_path: &str, x: f32, y: f32, rotation: f32, scale: f32) -> Self {
        let texture = window.load_texture(thread, texture_path).expect("Couldn't load texture");
        Self {
            texture,
            position: Vector2::new(x, y),
            rotation,
            scale,
        }
    }
}

impl GameObject for TexturedObject {
    fn draw(&self, draw_handle: &mut RaylibDrawHandle) {
        draw_handle.draw_texture_ex(
            &self.texture,
            self.position,
            self.rotation,
            self.scale,
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

    fn get_scale(&self) -> f32 {
        self.scale
    }

    fn set_scale(&mut self, scale: f32) {
        self.scale = scale;
    }

    fn update(&mut self) {
        // Add any update logic here
    }
}
