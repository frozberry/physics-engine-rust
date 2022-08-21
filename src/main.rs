extern crate sdl2;
mod application;
mod constants;
mod force;
mod graphics;
mod physics;

use application::Application;


fn main() {
    let mut app = Application::new();

    while app.is_running() {
        app.input();
        app.update();
        app.render();
    }

    app.destroy();
}
