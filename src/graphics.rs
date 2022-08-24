use crate::physics::vec2::Vec2;
use sdl2::{
    gfx::primitives::DrawRenderer,
    pixels::Color,
    rect::Rect,
    render::Canvas,
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

// C++ code uses floats, but I'm not sure why since it gets casted to a Sint16 anyway
// x - width / 2.0,

// I'm just going to use ints the whole time
// x - width / 2

// I'm also going to cast angles to i16

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

// pub fn draw_fill_polygon(x: i16, y: i16, vertices: Vec<Vec2>, color: u32) {
//     unsafe {
//         let mut vx: Vec<i16> = vec![];
//         let mut vy: Vec<i16> = vec![];

//         let vertices_len = vertices.len();

//         // Original code uses two seperate loops to do this - not sure why
//         for vertex in vertices {
//             vx.push(vertex.x as i16);
//             vy.push(vertex.y as i16);
//         }

//         filledPolygonColor(RENDERER, &vx[0], &vy[0], vertices_len as i32, color);
//         filledCircleColor(RENDERER, x, y, 1, 0xFF000000);
//     }
// }
// // This takes i32 as parameters since SDL_Rect wants c_ints (which are i32s).
// pub fn draw_texture(
//     x: i32,
//     y: i32,
//     width: i32,
//     height: i32,
//     rotation: f32,
//     texture: *mut SDL_Texture,
// ) {
//     unsafe {
//         let dst_rect = SDL_Rect {
//             x: x - (width / 2),
//             y: y - (height / 2),
//             w: width,
//             h: height,
//         };
//         let rotation_deg = rotation * 57.2958;

//         // Pass in null raw pointers
//         SDL_RenderCopyEx(
//             RENDERER,
//             texture,
//             ptr::null(),
//             &dst_rect,
//             rotation_deg as f64,
//             ptr::null(),
//             SDL_RendererFlip::SDL_FLIP_NONE,
//         );
//     }
// }
