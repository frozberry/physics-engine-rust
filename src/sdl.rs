use crate::application::{HEIGHT, WIDTH};
use sdl2::{pixels::Color, render::Canvas, video::Window, Sdl};

pub fn init_sdl() -> (Sdl, Canvas<Window>) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Graphics", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 64, 255));
    canvas.clear();
    canvas.present();

    return (sdl_context, canvas);
}
