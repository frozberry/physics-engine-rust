use super::vec2::Vec2;

pub struct Particle {
    pub pos: Vec2,
    pub vel: Vec2,
    pub acc: Vec2,
    pub mass: f32,
    pub radius: i16,
}

impl Particle {
    pub fn new(x: f32, y: f32, mass: f32) -> Self {
        Particle {
            pos: Vec2::new(x, y),
            vel: Vec2::new(0., 0.),
            acc: Vec2::new(0., 0.),
            mass,
            radius: 8,
        }
    }
}
