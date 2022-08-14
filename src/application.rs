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
        self.running = Graphics::OpenWindow()

        // Todo setup objects in scnee
    }

    pub fn input(&self) {
        todo!()
    }

    pub fn update(&self) {
        todo!()
    }

    pub fn render(&self) {
        Graphics::clear_screen(0xFF056263);
        Graphics::draw_fill_circle(200, 200, 40, 0xFFFFFFFF);
        Graphics::render_frame();
    }

    pub fn destroy(&self) {
        Graphics::close_window();
    }
}
