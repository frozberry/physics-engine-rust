use std::{
    thread::sleep,
    time::{Duration, SystemTime},
};

use sdl2::{
    event::Event, keyboard::Keycode, mouse::MouseButton, pixels::Color, render::Canvas,
    video::Window, Sdl,
};

use crate::{
    constants::MILLISECS_PER_FRAME,
    graphics::{self},
    physics::{body::Body, shape::Shape, vec2::Vec2, world::World},
    sdl::init_sdl,
};

pub struct Application {
    sdl: Sdl,
    canvas: Canvas<Window>,
    running: bool,
    time_previous_frame: SystemTime,
    debug: bool,
    gravity: bool,
    poly: bool,
    world: World,
}

impl Application {
    pub fn new() -> Self {
        let (sdl, canvas) = init_sdl();

        let mut a = Body::new(Shape::Box(300., 300.), 600., 800., 0.);
        a.restitution = 0.2;
        a.rotation = 0.7;
        a.add_texture("./assets/crate.png");
        let mut b = Body::new(Shape::Box(4000., 100.), 800., 1300., 0.);
        b.restitution = 0.6;

        let mut world = World::new(9.81);

        world.add_body(a);
        world.add_body(b);

        let application = Application {
            sdl,
            canvas,
            running: true,
            time_previous_frame: SystemTime::now(),
            debug: true,
            gravity: true,
            poly: true,
            world,
        };

        application
    }

    /* ---------------------------------- Input --------------------------------- */
    pub fn input(&mut self) {
        let mut event_pump = self.sdl.event_pump().unwrap();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    self.running = false;
                }
                Event::KeyDown { keycode, .. } => match keycode.unwrap() {
                    Keycode::Escape => {
                        self.running = false;
                    }
                    Keycode::D => self.debug = !self.debug,
                    Keycode::G => self.gravity = !self.gravity,
                    Keycode::P => self.poly = !self.poly,
                    _ => {}
                },
                Event::MouseButtonDown {
                    x, y, mouse_btn, ..
                } => match mouse_btn {
                    MouseButton::Left => {
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
                    MouseButton::Right => {
                        let mut p = Body::new(Shape::Circle(40.), x as f32, y as f32, 1.);
                        p.add_texture("./assets/basketball.png");
                        p.restitution = 0.8;
                        p.friction = 0.4;
                        self.world.add_body(p)
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }

    /* --------------------------------- Update --------------------------------- */
    pub fn update(&mut self) {
        graphics::clear_screen(Color::RGB(0, 64, 255), &mut self.canvas);

        let time_since_last_frame = SystemTime::now()
            .duration_since(self.time_previous_frame)
            .unwrap();
        let time_to_wait = MILLISECS_PER_FRAME - time_since_last_frame.as_millis() as i32;
        if time_to_wait > 0 {
            sleep(Duration::from_millis(time_to_wait as u64));
        }

        let now = SystemTime::now();

        let delta_time_ms = now
            .duration_since(self.time_previous_frame)
            .unwrap()
            .as_millis();

        let dt = f32::min(delta_time_ms as f32 / 1000., 0.016);

        self.world.update(dt, self.gravity, self.debug);
        self.time_previous_frame = now;
    }

    /* --------------------------------- Render --------------------------------- */

    pub fn render(&mut self) {
        // Draw bodies
        for body in self.world.get_bodies() {
            let color = if body.is_colliding && self.debug {
                Color::RED
            } else {
                Color::WHITE
            };
            match body.shape {
                Shape::Circle(radius) => {
                    if !self.debug && !body.texture.is_null() {
                        // graphics::draw_texture(
                        //     body.pos.x as i32,
                        //     body.pos.y as i32,
                        //     radius as i32 * 2,
                        //     radius as i32 * 2,
                        //     body.rotation,
                        //     body.texture,
                        // )
                    } else {
                        graphics::draw_circle(
                            body.pos.x as i16,
                            body.pos.y as i16,
                            radius as i16,
                            body.rotation,
                            color,
                            &mut self.canvas,
                        );
                    }
                }
                Shape::Box(width, height) => {
                    if !self.debug && !body.texture.is_null() {
                        // graphics::draw_texture(
                        //     body.pos.x as i32,
                        //     body.pos.y as i32,
                        //     w as i32,
                        //     h as i32,
                        //     body.rotation,
                        //     body.texture,
                        // );
                    } else {
                        graphics::draw_polygon(
                            body.pos.x as i16,
                            body.pos.y as i16,
                            body.shape.get_world_verticies(body.rotation, body.pos),
                            color,
                            &mut self.canvas,
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
                            &mut self.canvas,
                        );
                    } else {
                        graphics::draw_polygon(
                            body.pos.x as i16,
                            body.pos.y as i16,
                            body.shape.get_world_verticies(body.rotation, body.pos),
                            color,
                            &mut self.canvas,
                        );
                    }
                }
            }
        }

        self.canvas.present();
    }

    pub fn is_running(&self) -> bool {
        self.running
    }
}
