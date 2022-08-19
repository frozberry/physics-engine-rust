use super::{shape::Shape, vec2::Vec2};

pub struct Body {
    pub shape: Shape,
    pub is_colliding: bool,

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
}

impl Body {
    pub fn new(shape: Shape, x: f32, y: f32, mass: f32) -> Self {
        let inertia = shape.calc_inertia(mass);
        let inv_inertia = if inertia > 0. { 1. / inertia } else { 0. };
        let inv_mass = if mass > 0. { 1. / mass } else { 0. };

        Body {
            shape,
            is_colliding: false,
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
        }
    }

    pub fn update(&mut self, dt: f32) {
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

    fn clear_forces(&mut self) {
        self.net_force.x = 0.;
        self.net_force.y = 0.;
    }

    fn clear_torque(&mut self) {
        self.net_torque = 0.;
    }
}
