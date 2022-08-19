extern crate sdl2;
mod application;
mod constants;
mod force;
mod graphics;
mod physics;

use application::Application;

struct Foo {
    bar: i32,
}
impl Foo {
    fn my_method(&self) {
        let var = self.bar;
    }
}

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
