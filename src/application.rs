use std::mem::MaybeUninit;

use sdl2::sys::{
    SDL_Delay, SDL_Event, SDL_GetMouseState, SDL_GetTicks, SDL_PollEvent, SDL_Rect, SDL_BUTTON_LEFT,
};

use crate::{
    constants::*,
    force::{
        generate_drag_force, generate_friction_force, generate_gravitational_force,
        generate_spring_force, generate_spring_force_particles,
    },
    graphics::{self, height},
    physics::{
        body::{self, Body},
        vec2::Vec2,
    },
};

pub struct Application {
    running: bool,
    time_previous_frame: u32,
    particles: Vec<Body>,
    push_force: Vec2,
    liquid: SDL_Rect,
    mouse_cursor: Vec2,
    left_mouse_button_down: bool,
    anchor: Vec2,
    k: f32,
    rest_length: f32,
    num: usize,
}

impl Application {
    pub fn new() -> Self {
        let rect = SDL_Rect {
            x: 0,
            y: graphics::height() / 2,
            w: graphics::width(),
            h: graphics::height(),
        };

        let num = 10;

        let mut application = Application {
            running: false,
            time_previous_frame: 0,
            particles: vec![],
            push_force: Vec2::new(0., 0.),
            liquid: rect,
            mouse_cursor: Vec2::new(0., 0.),
            left_mouse_button_down: false,
            anchor: Vec2::new(1440., 200.),
            k: 200.,
            num,
            rest_length: 50.,
        };
        for i in 0..num {
            let p = Body::new(1440., 200. + i as f32 * 100., 1., 4);
            application.particles.push(p);
        }

        application
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
                    // This is really slow on WSL, I think because of X Server
                    // SDL_MOUSEMOTION => {
                    // if self.left_mouse_button_down {
                    //     self.mouse_cursor.x = event.motion.x as f32;
                    //     self.mouse_cursor.y = event.motion.y as f32;
                    // }
                    //     break;
                    // }
                    SDL_MOUSEBUTTONDOWN => {
                        // Code for spawning particles
                        // if event.button.button == SDL_BUTTON_LEFT as u8 {
                        //     let mut x = 1;
                        //     let mut y = 1;
                        //     SDL_GetMouseState(&mut x, &mut y);
                        //     let p = Particle::new(x as f32, y as f32, 1.);
                        //     self.particles.push(p);
                        // }
                        if !self.left_mouse_button_down
                            && event.button.button == SDL_BUTTON_LEFT as u8
                        {
                            self.left_mouse_button_down = true;
                            let mut x = 1;
                            let mut y = 1;
                            SDL_GetMouseState(&mut x, &mut y);
                            self.mouse_cursor.x = x as f32;
                            self.mouse_cursor.y = y as f32;
                        }

                        break;
                    }
                    SDL_MOUSEBUTTONUP => {
                        if self.left_mouse_button_down
                            && event.button.button == SDL_BUTTON_LEFT as u8
                        {
                            self.left_mouse_button_down = false;
                            let distance = self.particles[self.num - 1].pos - self.mouse_cursor;
                            let impulse_direction = distance.unit_vector();
                            let impulse_magnitude = distance.magnitude() * 5.0;
                            self.particles[self.num - 1].vel =
                                impulse_direction * impulse_magnitude;
                        }
                        break;
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

        // let attraction =
        //     generate_gravitational_force(&self.particles[0], &self.particles[1], 0.4, 5., 100.);
        // self.particles[0].add_force(attraction);
        // self.particles[1].add_force(-attraction);

        for particle in &mut self.particles {
            let drag = generate_drag_force(particle, 0.001);
            particle.add_force(drag);

            let weight = Vec2::new(0.0, particle.mass * 9.8 * PIXELS_PER_METER);
            particle.add_force(weight);

            particle.add_force(self.push_force);

            particle.integrate(delta_time);
            particle.clear_forces();
        }

        let spring_force =
            generate_spring_force(&self.particles[0], self.anchor, self.rest_length, self.k);
        self.particles[0].add_force(spring_force);

        for i in 1..self.num {
            let sf = generate_spring_force_particles(
                &self.particles[i],
                &self.particles[i - 1],
                self.rest_length,
                self.k,
            );
            self.particles[i].add_force(sf);
            self.particles[i - 1].add_force(-sf);
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

        if self.left_mouse_button_down {
            graphics::draw_line(
                self.particles[self.num - 1].pos.x as i16,
                self.particles[self.num - 1].pos.y as i16,
                self.mouse_cursor.x as i16,
                self.mouse_cursor.y as i16,
                0xFF0000FF,
            );
        }

        graphics::draw_line(
            self.anchor.x as i16,
            self.anchor.y as i16,
            self.particles[0].pos.x as i16,
            self.particles[0].pos.y as i16,
            0xFFFFFFFF,
        );

        for i in 0..(self.num - 1) {
            graphics::draw_line(
                self.particles[i].pos.x as i16,
                self.particles[i].pos.y as i16,
                self.particles[i + 1].pos.x as i16,
                self.particles[i + 1].pos.y as i16,
                0xFFFFFFFF,
            );
        }

        graphics::draw_fill_circle(
            self.anchor.x as i16,
            self.anchor.y as i16,
            5,
            0.,
            0xFFFFFFFF,
        );

        // graphics::draw_fill_rect(
        //     (self.liquid.x + self.liquid.w / 2) as i16,
        //     (self.liquid.y + self.liquid.h / 2) as i16,
        //     self.liquid.w as i16,
        //     self.liquid.h as i16,
        //     0xFF6E3713,
        // );

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
