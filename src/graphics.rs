use crate::physics::vec2::Vec2;
use sdl2::render::Texture;

const WINDOW_HEIGHT: i32 = 0;
const WINDOW_WIDTH: i32 = 0;

pub fn width() -> i32 {
    WINDOW_WIDTH
}
pub fn height() -> i32 {
    WINDOW_HEIGHT
}

pub fn open_window() -> bool {
    true
}
pub fn close_window() {}
pub fn clear_screen(color: u32) {}
pub fn render_frame() {}
pub fn draw_line(x0: i32, y0: i32, x1: i32, y1: i32, color: u32) {}
pub fn draw_circle(x: i32, y: i32, radius: i32, angle: f32, color: u32) {}
pub fn draw_fill_circle(x: i32, y: i32, radius: i32, angle: f32, color: u32) {}
pub fn draw_rect(x: i32, y: i32, width: i32, height: i32, color: u32) {}
pub fn draw_fill_rect(x: i32, y: i32, width: i32, height: i32, color: u32) {}

// vertices might need to be a slice
pub fn draw_polygon(x: i32, y: i32, vertices: Vec<Vec2>, color: u32) {}
pub fn draw_fill_polygon(x: i32, y: i32, vertices: Vec<Vec2>, color: u32) {}
pub fn draw_texture(x: i32, width: i32, height: i32, rotation: f32, texture: &Texture) {}
