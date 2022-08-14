use crate::graphics;

pub struct Application {
    running: bool,
}

impl Application {
    pub fn new() -> Self {
        Application { running: false }
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn setup(&mut self) {
        self.running = graphics::open_window()

        // Todo setup objects in scnee
    }

    pub fn input(&self) {
        todo!()
    }

    pub fn update(&self) {
        todo!()
    }

    pub fn render(&self) {
        graphics::clear_screen(0xFF056263);
        graphics::draw_fill_circle(200, 200, 40, 0., 0xFFFFFFFF);
        graphics::render_frame();
    }

    pub fn destroy(&self) {
        graphics::close_window();
    }
}
