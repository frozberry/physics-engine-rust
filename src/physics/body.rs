use super::vec2::Vec2;

struct Polygon {
    vertices: Vec<f32>,
}

pub enum Shape {
    // Circle(radius)
    Circle(f32),

    // Polygon(Vec<vertices>)
    Polygon(Vec<Vec2>),

    // Box<width, height, x, y>
    Box(f32, f32),
}

pub struct Body {
    pub pos: Vec2,
    pub vel: Vec2,
    pub acc: Vec2,
    pub mass: f32,
    pub inv_mass: f32,
    pub net_force: Vec2,
    pub shape: Shape,
    pub rotation: f32,
    pub ang_vel: f32,
    pub ang_acc: f32,
    pub inertia: f32,
    pub inv_inertia: f32,
    pub net_torque: f32,
}

impl Body {
    pub fn new(x: f32, y: f32, mass: f32, shape: Shape) -> Self {
        let inv_mass = if mass > 0. { 1. / mass } else { 0. };

        let inertia = match shape {
            Shape::Circle(radius) => mass * radius * radius * 0.5,
            _ => 0.,
        };
        let inv_inertia = if inertia > 0. { 1. / inertia } else { 0. };

        Body {
            pos: Vec2::new(x, y),
            vel: Vec2::new(0., 0.),
            acc: Vec2::new(0., 0.),
            mass,
            inv_mass,
            net_force: Vec2::new(0., 0.),
            shape,
            rotation: 0.,
            ang_vel: 0.,
            ang_acc: 0.,
            inertia,
            inv_inertia,
            net_torque: 0.,
        }
    }

    pub fn integrate_linear(&mut self, dt: f32) {
        self.acc = self.net_force * self.inv_mass;
        self.vel += self.acc * dt;
        self.pos += self.vel * dt;

        self.clear_forces();
    }

    pub fn integrate_angular(&mut self, dt: f32) {
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

    pub fn get_verticies(&self) -> Option<Vec<Vec2>> {
        match &self.shape {
            Shape::Circle(_) => None,
            Shape::Polygon(vertices) => Some(vertices.to_vec()),
            Shape::Box(w, h) => {
                let x = self.pos.x;
                let y = self.pos.y;
                let a = Vec2::new(x + w / 2., y + h / 2.);
                let b = Vec2::new(x + w / 2., y - h / 2.);
                let c = Vec2::new(x - w / 2., y + h / 2.);
                let d = Vec2::new(x - w / 2., y - h / 2.);
                Some(vec![a, b, c, d])
            }
        }
    }

    fn clear_forces(&mut self) {
        self.net_force.x = 0.;
        self.net_force.y = 0.;
    }

    fn clear_torque(&mut self) {
        self.net_torque = 0.;
    }
}
