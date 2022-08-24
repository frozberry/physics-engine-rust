use crate::constants::{HEIGHT, WIDTH};
use sdl2::{
    image::LoadTexture,
    pixels::Color,
    render::{Canvas, Texture},
    video::Window,
    Sdl,
};

pub fn init_sdl() -> (Sdl, Canvas<Window>, Texture, Texture, Texture) {
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

    let texture_creator = canvas.texture_creator();
    let basketball_texture = texture_creator
        .load_texture("./assets/basketball.png")
        .expect("Could not load texture");
    let crate_texture = texture_creator
        .load_texture("./assets/crate.png")
        .expect("Could not load texture");
    let bowlingball_texture = texture_creator
        .load_texture("./assets/bowlingball.png")
        .expect("Could not load texture");

    return (
        sdl_context,
        canvas,
        basketball_texture,
        crate_texture,
        bowlingball_texture,
    );
}
