use crate::physics::vec2::Vec2;
use sdl2::{
    gfx::primitives::DrawRenderer,
    pixels::Color,
    rect::Rect,
    render::{Canvas, Texture},
    sys::gfx::primitives::{
        boxColor, circleColor, filledCircleColor, filledPolygonColor, lineColor,
    },
    video::Window,
};

pub fn clear_screen(color: Color, canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(color);
    canvas.clear();
    // canvas.present();
}

pub fn draw_line(x1: i16, y1: i16, x2: i16, y2: i16, color: Color, canvas: &mut Canvas<Window>) {
    canvas.line(x1, y1, x2, y2, color).unwrap();
}

pub fn draw_circle(
    x: i16,
    y: i16,
    radius: i16,
    angle: f32,
    color: Color,
    canvas: &mut Canvas<Window>,
) {
    // This was previously causing issues since I was casting angle.cos() to i16. This lost
    // precision and roudned it down to zero. Doing the calculations with f32s fixes this.
    let d_x = radius as f32 * angle.cos();
    let d_y = radius as f32 * angle.sin();
    let x2 = x as f32 + d_x;
    let y2 = y as f32 + d_y;

    canvas.circle(x, y, radius, color).unwrap();
    canvas.line(x, y, x2 as i16, y2 as i16, color).unwrap();
}

pub fn draw_fill_circle(
    x: i16,
    y: i16,
    radius: i16,
    _angle: f32,
    color: Color,
    canvas: &mut Canvas<Window>,
) {
    canvas.circle(x, y, radius, color);
}

pub fn draw_rect(
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    color: Color,
    canvas: &mut Canvas<Window>,
) {
    canvas.set_draw_color(color);
    let rect = Rect::new(x, y, width, height);
    canvas.draw_rect(rect).unwrap();
}

pub fn draw_fill_rect(
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    color: Color,
    canvas: &mut Canvas<Window>,
) {
    canvas.set_draw_color(color);
    let rect = Rect::new(x, y, width, height);
    canvas.fill_rect(rect).unwrap();
}

pub fn draw_polygon(
    x: i16,
    y: i16,
    vertices: Vec<Vec2>,
    color: Color,
    canvas: &mut Canvas<Window>,
) {
    let vertices_len = vertices.len();

    for i in 0..vertices_len {
        let current_index = i;
        let next_index = (i + 1) % vertices_len;
        canvas
            .line(
                vertices[current_index].x as i16,
                vertices[current_index].y as i16,
                vertices[next_index].x as i16,
                vertices[next_index].y as i16,
                color,
            )
            .unwrap();
    }
    canvas.filled_circle(x, y, 1, color).unwrap();
}

pub fn draw_fill_polygon(
    x: i16,
    y: i16,
    vertices: Vec<Vec2>,
    color: Color,
    canvas: &mut Canvas<Window>,
) {
    let mut vx: Vec<i16> = vec![];
    let mut vy: Vec<i16> = vec![];

    for vertex in vertices {
        vx.push(vertex.x as i16);
        vy.push(vertex.y as i16);
    }

    canvas.filled_polygon(&vx, &vy, color).unwrap();
    canvas.filled_circle(x, y, 1, Color::BLACK).unwrap();
}

pub fn draw_texture(
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    rotation: f32,
    texture: &Texture,
    canvas: &mut Canvas<Window>,
) {
    let rect = Rect::new(
        x - (width / 2) as i32,
        y - (height / 2) as i32,
        width,
        height,
    );
    let rotation_deg = rotation * 57.2958;
    canvas
        .copy_ex(
            &texture,
            None,
            rect,
            rotation_deg as f64,
            None,
            false,
            false,
        )
        .unwrap();
}
