
use macroquad::prelude::*;

pub struct Player {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    s: f32,
}

impl Player {
    pub fn new(x:f32, y:f32, w:f32, h:f32, s:f32) -> Self {
        Self {x: x, y: y, w: w, h: h, s:s}
    }

    pub fn draw(&self, color: Color) {
        draw_rectangle(self.x, self.y, self.w, self.h, color);
    }

    pub fn update(&mut self, dt: f32) {
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            self.x -= self.s * dt
        } else if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            self.x += self.s * dt
        }
        self.x = clamp(self.x, 0.0, 600.0-self.w)
    }
}