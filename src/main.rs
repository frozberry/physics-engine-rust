mod application;
mod graphics;
mod physics;

use application::Application;

fn main() {
    let app = Application::new();
    while app.is_running() {
        app.input();
        app.update();
        app.render();
    }

    app.destroy();
}
