extern crate sdl2;
mod application;
mod graphics;
mod physics;

use application::Application;

fn main() {
    let mut app = Application::new();
    app.setup();
    // while app.is_running() {
    //     app.input();
    //     app.update();
    //     app.render();
    // }

    // let foo: Vec<u8> = vec![3];

    // app.destroy();
}
