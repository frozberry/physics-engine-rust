use crate::my_texture::MyTexture;

use super::{shape::Shape, vec2::Vec2};
use sdl2::sys::SDL_Texture;
use std::{os::raw::c_char, ptr};
extern "C" {
    fn puts(s: *const c_char);
}

#[derive(Clone)]
pub struct Body {
    pub shape: Shape,
    pub is_colliding: bool,
    pub is_static: bool,
    pub restitution: f32,
    pub friction: f32,

    pub pos: Vec2,
    pub vel: Vec2,
    pub acc: Vec2,
    pub mass: f32,
    pub inv_mass: f32,
    pub net_force: Vec2,

    pub rotation: f32,
    pub ang_vel: f32,
    pub ang_acc: f32,
    pub inertia: f32,
    pub inv_inertia: f32,
    pub net_torque: f32,

    pub texture: Option<MyTexture>,
}

impl Body {
    pub fn new<T: Into<Option<MyTexture>>>(
        shape: Shape,
        x: f32,
        y: f32,
        mass: f32,
        texture: T,
    ) -> Self {
        let inertia = shape.calc_inertia(mass);
        let inv_inertia = if inertia > 0. { 1. / inertia } else { 0. };
        let inv_mass = if mass != 0. { 1. / mass } else { 0. };

        let epsilon = 0.00005;
        let is_static = inv_mass < epsilon;

        Body {
            shape,
            is_colliding: false,
            is_static,
            restitution: 1.0,
            friction: 0.7,
            pos: Vec2::new(x, y),
            vel: Vec2::new(0., 0.),
            acc: Vec2::new(0., 0.),
            mass,
            inv_mass,
            net_force: Vec2::new(0., 0.),
            rotation: 0.,
            ang_vel: 0.,
            ang_acc: 0.,
            inertia,
            inv_inertia,
            net_torque: 0.,
            texture: texture.into(),
        }
    }

    pub fn update(&mut self, dt: f32) {
        if self.is_static {
            return;
        }
        self.integrate_linear(dt);
        self.integrate_angular(dt);
    }

    fn integrate_linear(&mut self, dt: f32) {
        self.acc = self.net_force * self.inv_mass;
        self.vel += self.acc * dt;
        self.pos += self.vel * dt;

        self.clear_forces();
    }

    fn integrate_angular(&mut self, dt: f32) {
        self.ang_acc += self.net_torque * self.inv_inertia;
        self.ang_vel += self.ang_acc * dt;
        self.rotation += self.ang_vel * dt;

        self.clear_torque();
    }

    pub fn add_force(&mut self, force: Vec2) {
        self.net_force += force;
    }

    pub fn add_torque(&mut self, torque: f32) {
        self.net_torque += torque;
    }

    pub fn apply_impulse(&mut self, impulse: Vec2, r: Vec2) {
        if self.is_static {
            return;
        }
        self.vel += impulse * self.inv_mass;
        self.ang_vel += r.cross(impulse) * self.inv_inertia;
    }

    fn clear_forces(&mut self) {
        self.net_force.x = 0.;
        self.net_force.y = 0.;
    }

    fn clear_torque(&mut self) {
        self.net_torque = 0.;
    }
}
