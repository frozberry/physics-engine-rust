use std::mem::MaybeUninit;

use sdl2::sys::{
    SDL_Delay, SDL_Event, SDL_GetMouseState, SDL_GetTicks, SDL_PollEvent, SDL_BUTTON_LEFT,
    SDL_BUTTON_RIGHT,
};

use crate::{
    constants::*,
    graphics::{self},
    physics::{body::Body, shape::Shape, vec2::Vec2, world::World},
};

pub struct Application {
    running: bool,
    time_previous_frame: u32,
    debug: bool,
    gravity: bool,
    poly: bool,
    world: World,
}

impl Application {
    pub fn new() -> Self {
        let running = graphics::open_window();

        let mut a = Body::new(Shape::Box(300., 300.), 600., 800., 0.);
        a.restitution = 0.2;
        a.rotation = 0.7;
        a.add_texture("./assets/crate.png");
        let mut b = Body::new(Shape::Box(4000., 100.), 800., 1300., 0.);
        b.restitution = 0.6;

        let mut world = World::new(9.81);

        world.add_body(a);
        world.add_body(b);

        let wind = Vec2::new(0.5 * PIXELS_PER_METER, 0.);
        // world.add_force(wind);

        let application = Application {
            running,
            time_previous_frame: 0,
            debug: false,
            gravity: true,
            poly: false,
            world,
        };

        application
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    /* ---------------------------------- Input --------------------------------- */

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
                            SDLK_D => self.debug = !self.debug,
                            SDLK_G => self.gravity = !self.gravity,
                            SDLK_P => self.poly = !self.poly,
                            _ => {}
                        }
                        break;
                    }
                    SDL_MOUSEBUTTONDOWN => {
                        // Code for spawning particles
                        if event.button.button == SDL_BUTTON_LEFT as u8 {
                            let mut x = 1;
                            let mut y = 1;
                            SDL_GetMouseState(&mut x, &mut y);
                            let v = vec![
                                Vec2::new(20., 60.),
                                Vec2::new(-40., 20.),
                                Vec2::new(-20., -60.),
                                Vec2::new(20., -60.),
                                Vec2::new(40., 20.),
                            ];
                            let mut p;
                            if self.poly {
                                p = Body::new(Shape::Polygon(v), x as f32, y as f32, 1.);
                            } else {
                                p = Body::new(Shape::Box(100., 100.), x as f32, y as f32, 1.);
                                p.add_texture("./assets/crate.png");
                            }
                            p.restitution = 0.3;
                            p.friction = 0.4;
                            self.world.add_body(p)
                        }
                        if event.button.button == SDL_BUTTON_RIGHT as u8 {
                            let mut x = 1;
                            let mut y = 1;
                            SDL_GetMouseState(&mut x, &mut y);
                            let mut p = Body::new(Shape::Circle(40.), x as f32, y as f32, 1.);
                            p.add_texture("./assets/basketball.png");
                            p.restitution = 0.8;
                            p.friction = 0.4;
                            self.world.add_body(p)
                        }

                        // Code for pool effect
                        // if !self.left_mouse_button_down
                        //     && event.button.button == SDL_BUTTON_LEFT as u8
                        // {
                        // self.left_mouse_button_down = true;
                        //     let mut x = 1;
                        //     let mut y = 1;
                        //     SDL_GetMouseState(&mut x, &mut y);
                        //     self.mouse_cursor.x = x as f32;
                        //     self.mouse_cursor.y = y as f32;
                        // }

                        break;
                    }
                    // SDL_MOUSEBUTTONUP => {
                    //     if self.left_mouse_button_down
                    //         && event.button.button == SDL_BUTTON_LEFT as u8
                    //     {
                    //         self.left_mouse_button_down = false;
                    //         let distance = self.bodies[0].pos - self.mouse_cursor;
                    //         let impulse_direction = distance.unit_vector();
                    //         let impulse_magnitude = distance.magnitude() * 5.0;
                    //         self.bodies[0].vel = impulse_direction * impulse_magnitude;
                    //     }
                    //     break;
                    // }
                    _ => {}
                }
            }
        }
    }

    /* --------------------------------- Update --------------------------------- */

    pub fn update(&mut self) {
        graphics::clear_screen(0xFF056263);
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
        self.world.update(delta_time, self.gravity, self.debug);
        self.time_previous_frame = sdl_ticks;
    }

    /* --------------------------------- Render --------------------------------- */

    pub fn render(&self) {
        // Draw bodies
        for body in self.world.get_bodies() {
            let color = if body.is_colliding && self.debug {
                0xFF0000FF
            } else {
                0xFFFFFFFF
            };
            match body.shape {
                Shape::Circle(radius) => {
                    if !self.debug && !body.texture.is_null() {
                        graphics::draw_texture(
                            body.pos.x as i32,
                            body.pos.y as i32,
                            radius as i32 * 2,
                            radius as i32 * 2,
                            body.rotation,
                            body.texture,
                        )
                    } else {
                        graphics::draw_circle(
                            body.pos.x as i16,
                            body.pos.y as i16,
                            radius as i16,
                            body.rotation,
                            color,
                        );
                    }
                }
                Shape::Box(w, h) => {
                    if !self.debug && !body.texture.is_null() {
                        graphics::draw_texture(
                            body.pos.x as i32,
                            body.pos.y as i32,
                            w as i32,
                            h as i32,
                            body.rotation,
                            body.texture,
                        );
                    } else {
                        graphics::draw_polygon(
                            body.pos.x as i16,
                            body.pos.y as i16,
                            body.shape.get_world_verticies(body.rotation, body.pos),
                            color,
                        );
                    }
                }
                Shape::Polygon(_) => {
                    if !self.debug {
                        graphics::draw_fill_polygon(
                            body.pos.x as i16,
                            body.pos.y as i16,
                            body.shape.get_world_verticies(body.rotation, body.pos),
                            color,
                        );
                    } else {
                        graphics::draw_polygon(
                            body.pos.x as i16,
                            body.pos.y as i16,
                            body.shape.get_world_verticies(body.rotation, body.pos),
                            color,
                        );
                    }
                }
            }
        }
        graphics::render_frame();
    }

    pub fn destroy(&self) {
        graphics::close_window();
    }
}
