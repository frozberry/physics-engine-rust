use std::mem::MaybeUninit;

use sdl2::sys::{
    SDL_Delay, SDL_Event, SDL_GetMouseState, SDL_GetTicks, SDL_PollEvent, SDL_Rect, SDL_BUTTON_LEFT,
};

use crate::{
    constants::*,
    force::{
        generate_drag_force, generate_friction_force, generate_gravitational_force,
        generate_spring_force, generate_spring_force_bodies,
    },
    graphics::{self, height},
    physics::{
        body::{self, Body, Shape},
        vec2::Vec2,
    },
};

pub struct Application {
    running: bool,
    time_previous_frame: u32,
    bodies: Vec<Body>,
    push_force: Vec2,
    mouse_cursor: Vec2,
    left_mouse_button_down: bool,
}

impl Application {
    pub fn new() -> Self {
        let a = Body::new(200., 200., 1., Shape::Circle(50.));
        let b = Body::new(600., 200., 3., Shape::Circle(80.));
        let c = Body::new(500., 500., 1., Shape::Box(10., 40.));

        let mut application = Application {
            running: false,
            time_previous_frame: 0,
            bodies: vec![],
            push_force: Vec2::new(0., 0.),
            mouse_cursor: Vec2::new(0., 0.),
            left_mouse_button_down: false,
        };

        application.bodies.push(a);
        application.bodies.push(b);
        application.bodies.push(c);
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
                            let distance = self.bodies[0].pos - self.mouse_cursor;
                            let impulse_direction = distance.unit_vector();
                            let impulse_magnitude = distance.magnitude() * 5.0;
                            self.bodies[0].vel = impulse_direction * impulse_magnitude;
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

        for body in &mut self.bodies {
            let drag = generate_drag_force(body, 0.001);
            body.add_force(drag);

            let weight = Vec2::new(0.0, body.mass * 9.8 * PIXELS_PER_METER);
            // body.add_force(weight);

            body.add_force(self.push_force);

            let torque = 20.;
            body.add_torque(torque);

            body.integrate_linear(delta_time);
            body.integrate_angular(delta_time);
        }

        let win_height = graphics::height() as f32;
        let win_width = graphics::width() as f32;

        for body in &mut self.bodies {
            match body.shape {
                Shape::Circle(radius) => {
                    if body.pos.y > win_height {
                        body.pos.y = win_height - radius;
                        body.vel.y *= -0.9
                    }
                    if body.pos.y < 0. {
                        body.pos.y = radius;
                        body.vel.y *= -0.9
                    }

                    if body.pos.x > win_width {
                        body.pos.x = win_width - radius;
                        body.vel.x *= -0.9;
                    }
                    if body.pos.x < 0. {
                        body.pos.x = radius;
                        body.vel.x *= -0.9;
                    }
                }
                _ => {}
            }
        }

        self.time_previous_frame = sdl_ticks;
    }

    pub fn render(&self) {
        graphics::clear_screen(0xFF056263);

        if self.left_mouse_button_down {
            graphics::draw_line(
                self.bodies[0].pos.x as i16,
                self.bodies[0].pos.y as i16,
                self.mouse_cursor.x as i16,
                self.mouse_cursor.y as i16,
                0xFF0000FF,
            );
        }

        // Draw bodies
        for body in &self.bodies {
            match body.shape {
                Shape::Circle(radius) => {
                    graphics::draw_circle(
                        body.pos.x as i16,
                        body.pos.y as i16,
                        radius as i16,
                        body.rotation,
                        0xFFFFFFFF,
                    );
                }
                Shape::Box(w, h) => graphics::draw_fill_rect(
                    body.pos.x as i16,
                    body.pos.y as i16,
                    w as i16,
                    h as i16,
                    0xFFFFFFFF,
                ),
                _ => {}
            }
        }
        graphics::render_frame();
    }

    pub fn destroy(&self) {
        graphics::close_window();
    }
}
