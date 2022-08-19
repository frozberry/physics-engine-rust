extern crate sdl2;
mod application;
mod constants;
mod force;
mod graphics;
mod physics;

use application::Application;
use physics::{body::Body, contact::Contact, shape::Shape, vec2::Vec2};

fn main() {
    let mut app = Application::new();
    app.setup();

    while app.is_running() {
        app.input();
        app.update();
        app.render();
    }

    app.destroy();
}
