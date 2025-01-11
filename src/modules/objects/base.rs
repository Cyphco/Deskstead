use raylib::prelude::*;

pub trait GameObject {
    fn draw(&self, draw_handle: &mut RaylibDrawHandle);
    fn get_position(&self) -> Vector2;
    fn set_position(&mut self, position: Vector2);
    fn get_rotation(&self) -> f32;
    fn set_rotation(&mut self, rotation: f32);
    fn get_scale(&self) -> f32;
    fn set_scale(&mut self, scale: f32);
    fn update(&mut self);
}
