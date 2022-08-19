use super::{body::Body, vec2::Vec2};

#[derive(Debug)]
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

    pub fn resolve_collision(&mut self) {
        self.resolve_penetration();

        let e = f32::min(self.a.restitution, self.b.restitution);
        let v_rel = self.a.vel - self.b.vel;
        println!("v_rel: {:?}", v_rel);
        let v_rel_dot_normal = v_rel.dot(self.normal);

        let impulse_magnitude = -(1. + e) * v_rel_dot_normal / (self.a.inv_mass + self.b.inv_mass);
        let impulse_direction = self.normal;
        let jn = impulse_direction * impulse_magnitude;

        self.a.apply_impulse(jn);
        self.b.apply_impulse(-jn);
    }
}
