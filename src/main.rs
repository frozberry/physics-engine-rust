extern crate sdl2;
mod application;
mod graphics;
mod physics;
mod sdl;

use application::Application;

fn main() {
    let mut app = Application::new();

    while app.is_running() {
        app.input();
        app.update();
        app.render();
    }
}
