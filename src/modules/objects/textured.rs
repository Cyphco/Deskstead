use raylib::prelude::*;
use super::base::GameObject;

pub struct TexturedObject {
    texture: Texture2D,
    position: Vector2,
    rotation: f32,
    size: Vector2,
}

impl TexturedObject {
    pub fn new(
        texture: Texture2D,
        position: Vector2,
        size: Vector2,
        rotation: f32,
    ) -> Self {
        Self {
            texture,
            position,
            rotation,
            size,
        }
    }
}

impl GameObject for TexturedObject {
    fn update(&mut self, _dt: f32) {
        // No update behavior needed for static textured objects
    }

    fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_texture_pro(
            &self.texture,
            Rectangle::new(0.0, 0.0, self.texture.width() as f32, self.texture.height() as f32),
            Rectangle::new(self.position.x, self.position.y, self.size.x, self.size.y),
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
