use super::{body::Body, vec2::Vec2};

pub struct Contact<'a> {
    pub a: &'a mut Body,
    pub b: &'a mut Body,
    pub start: Vec2,
    pub end: Vec2,
    pub normal: Vec2,
    pub depth: f32,
}

impl<'a> Contact<'a> {
    pub fn new(
        a: &'a mut Body,
        b: &'a mut Body,
        start: Vec2,
        end: Vec2,
        normal: Vec2,
        depth: f32,
    ) -> Self {
        Contact {
            a,
            b,
            start,
            end,
            normal,
            depth,
        }
    }
    pub fn resolve_penetration(&mut self) {
        if self.a.is_static && self.b.is_static {
            return;
        }

        self.a.is_colliding = true;
        self.b.is_colliding = true;
        let da: f32 = self.depth / (self.a.inv_mass + self.b.inv_mass) * self.a.inv_mass;
        let db: f32 = self.depth / (self.a.inv_mass + self.b.inv_mass) * self.b.inv_mass;
        self.a.pos -= self.normal * da;
        self.b.pos += self.normal * db;
    }
}
