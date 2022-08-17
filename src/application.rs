use std::{cmp::min, mem::MaybeUninit};

use sdl2::sys::{SDL_Delay, SDL_Event, SDL_EventType, SDL_GetTicks, SDL_KeyCode, SDL_PollEvent};

use crate::{
    constants::*,
    graphics::{self, height},
    physics::{
        particle::{self, Particle},
        vec2::Vec2,
    },
};

pub struct Application {
    running: bool,
    time_previous_frame: u32,
    // C++ uses a pointer to particle. I'm avoiding for now since it requires lifetimes in Rust
    particles: Vec<Particle>,
}

impl Application {
    pub fn new() -> Self {
        let small = Particle::new(50., 100., 1., 8);
        let big = Particle::new(50., 200., 4., 32);
        Application {
            running: false,
            time_previous_frame: 0,
            particles: vec![small, big],
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

    pub fn update(&mut self) {
        // Unsafe calls to SDL only
        unsafe {
            let time_since_last_frame = SDL_GetTicks() - self.time_previous_frame;
            let time_to_wait = MILLISECS_PER_FRAME - time_since_last_frame as i32;
            if time_to_wait > 0 {
                SDL_Delay(time_to_wait as u32)
            }
        }

        let sdl_ticks;
        unsafe {
            sdl_ticks = SDL_GetTicks();
        }

        let delta_time_ms = (sdl_ticks - self.time_previous_frame) as f32;
        let delta_time = f32::min(delta_time_ms / 1000., 0.016);

        for particle in &mut self.particles {
            let wind = Vec2::new(1. * PIXELS_PER_METER, 0.);
            let g = Vec2::new(0., 9.81 * PIXELS_PER_METER * particle.mass);

            particle.add_force(wind);
            particle.add_force(g);
            particle.integrate(delta_time);
            particle.clear_forces();
        }

        let win_height = graphics::height() as f32;
        let win_width = graphics::width() as f32;
        for particle in &mut self.particles {
            if particle.pos.y > win_height || particle.pos.y < 0. {
                particle.pos.y = win_height - particle.radius as f32;
                particle.vel.y *= -0.9
            }

            if particle.pos.x > win_width {
                particle.pos.x = win_width - particle.radius as f32;
                particle.vel.x *= -0.9;
            }
            if particle.pos.x < 0. {
                particle.pos.x = particle.radius as f32;
                particle.vel.x *= -0.9;
            }
        }

        self.time_previous_frame = sdl_ticks;
    }

    pub fn render(&self) {
        graphics::clear_screen(0xFF056263);
        // graphics::draw_fill_circle(200, 200, 40, 0., 0xFFFFFFFF);

        for particle in &self.particles {
            graphics::draw_fill_circle(
                particle.pos.x as i16,
                particle.pos.y as i16,
                particle.radius,
                0.,
                0xFFFFFFFF,
            );
        }
        graphics::render_frame();
    }

    pub fn destroy(&self) {
        graphics::close_window();
    }
}
