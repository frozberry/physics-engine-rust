use std::{cmp::min, mem::MaybeUninit};

use sdl2::sys::{
    SDL_Delay, SDL_Event, SDL_EventType, SDL_GetMouseState, SDL_GetTicks, SDL_KeyCode,
    SDL_PollEvent, SDL_Rect, SDL_BUTTON_LEFT,
};

use crate::{
    constants::*,
    force::generate_drag_force,
    graphics::{self, height},
    physics::{
        particle::{self, Particle},
        vec2::Vec2,
    },
};

pub struct Application {
    running: bool,
    time_previous_frame: u32,
    particles: Vec<Particle>,
    push_force: Vec2,
    liquid: SDL_Rect,
}

impl Application {
    pub fn new() -> Self {
        let small = Particle::new(50., 100., 1., 8);
        // let big = Particle::new(50., 200., 4., 32);

        let rect = SDL_Rect {
            x: 0,
            y: graphics::height() / 2,
            w: graphics::width(),
            h: graphics::height(),
        };

        Application {
            running: false,
            time_previous_frame: 0,
            particles: vec![small],
            push_force: Vec2::new(0., 0.),
            liquid: rect,
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

                // Uses constants defined in constants.rs because of SDL2 awkwardness
                match event.type_ {
                    SDLK_QUIT => {
                        self.running = false;
                        break;
                    }
                    SDLK_KEYDOWN => {
                        match event.key.keysym.sym {
                            SDLK_ESCAPE => {
                                self.running = false;
                            }
                            SDLK_UP => self.push_force.y = -50. * PIXELS_PER_METER,
                            SDLK_DOWN => self.push_force.y = 50. * PIXELS_PER_METER,
                            SDLK_LEFT => self.push_force.x = -50. * PIXELS_PER_METER,
                            SDLK_RIGHT => self.push_force.x = 50. * PIXELS_PER_METER,
                            _ => {}
                        }
                        break;
                    }
                    SDLK_KEYUP => {
                        match event.key.keysym.sym {
                            SDLK_UP => self.push_force.y = 0.,
                            SDLK_DOWN => self.push_force.y = 0.,
                            SDLK_LEFT => self.push_force.x = 0.,
                            SDLK_RIGHT => self.push_force.x = 0.,
                            _ => {}
                        }
                        break;
                    }
                    SDL_MOUSEBUTTONDOWN => {
                        if event.button.button == SDL_BUTTON_LEFT as u8 {
                            let mut x = 1;
                            let mut y = 1;
                            SDL_GetMouseState(&mut x, &mut y);
                            let p = Particle::new(x as f32, y as f32, 1., 4);
                            self.particles.push(p);
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
            // let wind = Vec2::new(1. * PIXELS_PER_METER, 0.);
            let g = Vec2::new(0., 9.81 * PIXELS_PER_METER * particle.mass);

            // particle.add_force(wind);
            particle.add_force(g);
            particle.add_force(self.push_force);

            if particle.pos.y > self.liquid.y as f32 {
                let drag = generate_drag_force(&particle, 0.01);
                particle.add_force(drag)
            }

            particle.integrate(delta_time);
            particle.clear_forces();
        }

        let win_height = graphics::height() as f32;
        let win_width = graphics::width() as f32;

        for particle in &mut self.particles {
            if particle.pos.y > win_height {
                particle.pos.y = win_height - particle.radius as f32;
                particle.vel.y *= -0.9
            }
            if particle.pos.y < 0. {
                particle.pos.y = particle.radius as f32;
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

        graphics::draw_fill_rect(
            (self.liquid.x + self.liquid.w / 2) as i16,
            (self.liquid.y + self.liquid.h / 2) as i16,
            self.liquid.w as i16,
            self.liquid.h as i16,
            0xFF6E3713,
        );

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
