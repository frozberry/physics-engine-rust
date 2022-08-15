use super::vec2::Vec2;

pub struct Particle {
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub mass: f32,
}

impl Particle {
    pub fn new(x: f32, y: f32, mass: f32) -> Self {
        Particle {
            position: Vec2::new(x, y),
            velocity: Vec2::new(0., 0.),
            acceleration: Vec2::new(0., 0.),
            mass,
        }
    }
}
