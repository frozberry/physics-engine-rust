use super::vec2::Vec2;

pub struct Particle {
    pub pos: Vec2,
    pub vel: Vec2,
    pub acc: Vec2,
    pub mass: f32,
    pub radius: i16,
    pub net_force: Vec2,
}

impl Particle {
    pub fn new(x: f32, y: f32, mass: f32, radius: i16) -> Self {
        Particle {
            pos: Vec2::new(x, y),
            vel: Vec2::new(0., 0.),
            acc: Vec2::new(0., 0.),
            mass,
            radius,
            net_force: Vec2::new(0., 0.),
        }
    }

    pub fn integrate(&mut self, dt: f32) {
        self.acc = self.net_force / self.mass;
        self.vel += self.acc * dt;
        self.pos += self.vel * dt;
    }

    pub fn add_force(&mut self, force: Vec2) {
        self.net_force += force;
    }

    pub fn clear_forces(&mut self) {
        self.net_force.x = 0.;
        self.net_force.y = 0.;
    }
}
