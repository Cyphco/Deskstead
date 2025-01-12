use raylib::prelude::*;

pub trait GameObject {
    fn draw(&self, d: &mut RaylibDrawHandle);
    fn update(&mut self, dt: f32);
    fn get_position(&self) -> Vector2;
    fn set_position(&mut self, position: Vector2);
    fn get_rotation(&self) -> f32;
    fn set_rotation(&mut self, rotation: f32);
    fn get_size(&self) -> Vector2;
}
