use std::{
    thread::sleep,
    time::{Duration, SystemTime},
};

use physics_engine::{
    body::Body, constants::MILLISECS_PER_FRAME, my_texture::MyTexture, shape::Shape, vec2::Vec2,
    world::World,
};
use rand::Rng;
use sdl2::{
    event::Event,
    keyboard::Keycode,
    mouse::MouseButton,
    pixels::Color,
    render::{Canvas, Texture},
    video::Window,
    Sdl,
};

use crate::{
    graphics::{self},
    sdl::init_sdl,
};

pub struct Application {
    sdl: Sdl,
    canvas: Canvas<Window>,
    running: bool,
    time_previous_frame: SystemTime,
    debug: bool,
    gravity: bool,
    wind: bool,
    polygon: bool,
    world: World,
    crate_texture: Texture,
    basketball_texture: Texture,
    bowlingball_texture: Texture,
    metal_texture: Texture,
}

impl Application {
    pub fn new() -> Self {
        let (sdl, canvas, basketball_texture, crate_texture, bowlingball_texture, metal_texture) =
            init_sdl();

        let mut world = World::new(9.81);

        let mut body = Body::new(Shape::Box(75., 75.), 350., 350., 0., MyTexture::Crate);
        body.restitution = 0.2;
        body.rotation = 0.7;

        let mut floor = Body::new(Shape::Box(4000., 50.), 1000., 750., 0., None);
        floor.restitution = 0.8;
        floor.texture = Some(MyTexture::Metal);

        let mut l_wall = Body::new(Shape::Box(50., 600.), 100., 450., 0., None);
        l_wall.restitution = 0.6;
        l_wall.texture = Some(MyTexture::Metal);
        let mut r_wall = Body::new(Shape::Box(50., 600.), 1100., 450., 0., None);
        r_wall.restitution = 0.6;
        r_wall.texture = Some(MyTexture::Metal);

        world.add_body(body);
        world.add_body(floor);
        world.add_body(r_wall);
        world.add_body(l_wall);

        Application {
            sdl,
            canvas,
            running: true,
            time_previous_frame: SystemTime::now(),
            debug: false,
            wind: false,
            gravity: true,
            polygon: false,
            world,
            crate_texture,
            basketball_texture,
            bowlingball_texture,
            metal_texture,
        }
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
                    Keycode::W => self.wind = !self.wind,
                    Keycode::P => self.polygon = !self.polygon,
                    _ => {}
                },
                Event::MouseButtonDown {
                    x, y, mouse_btn, ..
                } => match mouse_btn {
                    MouseButton::Left => {
                        let mut rng = rand::thread_rng();
                        let r: f64 = rng.gen();
                        if r > 0.5 {
                            self.world.add_body(Body::basketball(x as f32, y as f32));
                        } else {
                            self.world.add_body(Body::bowlingball(x as f32, y as f32));
                        }
                    }
                    MouseButton::Right => {
                        let v = vec![
                            Vec2::new(20., 60.),
                            Vec2::new(-40., 20.),
                            Vec2::new(-20., -60.),
                            Vec2::new(20., -60.),
                            Vec2::new(40., 20.),
                        ];
                        if self.polygon {
                            let p = Body::new(Shape::Polygon(v), x as f32, y as f32, 1., None);
                            self.world.add_body(p)
                        } else {
                            self.world.add_body(Body::crate_(x as f32, y as f32));
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }

    /* --------------------------------- Update --------------------------------- */
    pub fn update(&mut self) {
        graphics::clear_screen(Color::RGB(40, 40, 200), &mut self.canvas);

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

        self.world.update(dt, self.gravity, self.wind);
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
                    if self.debug || body.texture.is_none() {
                        graphics::draw_circle(
                            body.pos.x as i16,
                            body.pos.y as i16,
                            radius as i16,
                            body.rotation,
                            color,
                            &mut self.canvas,
                        );
                    } else {
                        let texture = match body.texture.unwrap() {
                            MyTexture::Crate => &self.crate_texture,
                            MyTexture::Metal => &self.metal_texture,
                            MyTexture::BowlingBall => &self.bowlingball_texture,
                            MyTexture::BasketBall => &self.basketball_texture,
                        };
                        graphics::draw_texture(
                            body.pos.x as i32,
                            body.pos.y as i32,
                            radius as u32 * 2,
                            radius as u32 * 2,
                            body.rotation,
                            texture,
                            &mut self.canvas,
                        )
                    }
                }
                Shape::Box(width, height) => {
                    if self.debug || body.texture.is_none() {
                        graphics::draw_polygon(
                            body.pos.x as i16,
                            body.pos.y as i16,
                            body.shape.get_world_verticies(body.rotation, body.pos),
                            color,
                            &mut self.canvas,
                        );
                    } else {
                        let texture = match body.texture.unwrap() {
                            MyTexture::Crate => &self.crate_texture,
                            MyTexture::Metal => &self.metal_texture,
                            MyTexture::BowlingBall => &self.bowlingball_texture,
                            MyTexture::BasketBall => &self.basketball_texture,
                        };
                        graphics::draw_texture(
                            body.pos.x as i32,
                            body.pos.y as i32,
                            width as u32,
                            height as u32,
                            body.rotation,
                            texture,
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
