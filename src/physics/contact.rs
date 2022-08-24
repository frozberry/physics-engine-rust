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

    pub fn resolve_collision(&mut self) {
        self.resolve_penetration();

        let e = f32::min(self.a.restitution, self.b.restitution);
        let f = f32::min(self.a.friction, self.b.friction);

        // Calculate the relavtive velocity between the bodies
        let ra = self.end - self.a.pos;
        let rb = self.start - self.b.pos;
        let va = self.a.vel + Vec2::new(-self.a.ang_vel * ra.y, self.a.ang_vel * ra.x);
        let vb = self.b.vel + Vec2::new(-self.b.ang_vel * rb.y, self.b.ang_vel * rb.x);
        let v_rel = va - vb;

        // Relative velocity along the collision normal
        let v_rel_dot_normal = v_rel.dot(self.normal);
        let impulse_direction_n = self.normal;
        let impulse_magnitude_n = -(1. + e) * v_rel_dot_normal
            / ((self.a.inv_mass + self.b.inv_mass)
                + ra.cross(self.normal) * ra.cross(self.normal) * self.a.inv_inertia
                + rb.cross(self.normal) * rb.cross(self.normal) * self.b.inv_inertia);
        let jn = impulse_direction_n * impulse_magnitude_n;

        // Relative velocity along the collision tangent
        let tangent = self.normal.normal();
        let v_rel_dot_tangent = v_rel.dot(tangent);
        let impulse_direction_t = tangent;
        let impulse_magnitude_t = f * -(1. + e) * v_rel_dot_tangent
            / ((self.a.inv_mass + self.b.inv_mass)
                + ra.cross(tangent) * ra.cross(tangent) * self.a.inv_inertia
                + rb.cross(tangent) * rb.cross(tangent) * self.b.inv_inertia);
        let jt = impulse_direction_t * impulse_magnitude_t;

        let impulse = jn + jt;

        self.a.apply_impulse(impulse, ra);
        self.b.apply_impulse(-impulse, rb);
    }
}
