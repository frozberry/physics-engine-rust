use std::mem::MaybeUninit;

use sdl2::sys::{SDL_Event, SDL_EventType, SDL_KeyCode, SDL_PollEvent};

use crate::{graphics, physics::particle::Particle};

pub struct Application {
    running: bool,
    // C++ uses a pointer to particle. I'm avoiding for now since it requires lifetimes in Rust
    particle: Particle,
}

impl Application {
    pub fn new() -> Self {
        let p = Particle::new(50., 100., 1.);
        Application {
            running: false,
            particle: p,
        }
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn setup(&mut self) {
        self.running = graphics::open_window()

        // Todo setup objects in scnee
    }

    // This is not the best way to handle inputs in Rust
    // I am trying to replicate the original C++ code
    pub fn input(&mut self) {
        unsafe {
            // Original C++ code uses uninitalized variable
            // See graphics.rs line 29 for explanation
            let mut uninit_event: MaybeUninit<SDL_Event> = MaybeUninit::uninit();

            // SDL_PollEvent returns 1 or 0
            // Original C++ just used implicit boolean conversion
            while SDL_PollEvent(uninit_event.as_mut_ptr()) != 0 {
                let event = uninit_event.assume_init();

                // This is bad code, but is needed since the sdl2 crate
                // only exposes events as enums. But event.type_ is a u32.
                // If I was using sdl2 the Rust way, this wouldn't be a problem
                // But I'm trying to replicate the C++ code as closely as possible
                match event.type_ {
                    e if e == SDL_EventType::SDL_QUIT as u32 => {
                        self.running = false;
                        break;
                    }
                    e if e == SDL_EventType::SDL_KEYDOWN as u32 => {
                        if event.key.keysym.sym == SDL_KeyCode::SDLK_ESCAPE as i32 {
                            self.running = false;
                            break;
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn update(&self) {
        // todo!()
    }

    pub fn render(&self) {
        graphics::clear_screen(0xFF056263);
        graphics::draw_fill_circle(200, 200, 40, 0., 0xFFFFFFFF);
        graphics::draw_fill_circle(
            self.particle.position.x as i16,
            self.particle.position.y as i16,
            1,
            0.,
            0xFFFFFFFF,
        );
        graphics::render_frame();
    }

    pub fn destroy(&self) {
        graphics::close_window();
    }
}
