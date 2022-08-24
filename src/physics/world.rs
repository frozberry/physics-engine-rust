use crate::{
    constants::PIXELS_PER_METER,
    graphics,
    physics::{body::Body, collision, vec2::Vec2},
};

pub struct World {
    g: f32,
    bodies: Vec<Body>,
    forces: Vec<Vec2>,
    torques: Vec<f32>,
}

impl World {
    pub fn new(g: f32) -> Self {
        World {
            g,
            bodies: vec![],
            forces: vec![],
            torques: vec![],
        }
    }
    pub fn add_body(&mut self, body: Body) {
        self.bodies.push(body);
    }

    pub fn get_bodies(&self) -> Vec<Body> {
        self.bodies.clone()
    }

    pub fn add_force(&mut self, force: Vec2) {
        self.forces.push(force);
    }
    pub fn add_torque(&mut self, torque: f32) {
        self.torques.push(torque);
    }

    pub fn update(&mut self, dt: f32, gravity: bool, debug: bool) {
        for body in &mut self.bodies {
            if gravity {
                let weight = Vec2::new(0.0, body.mass * self.g * PIXELS_PER_METER);
                body.add_force(weight);
            }
            for force in self.forces.clone() {
                body.add_force(force)
            }
            for torque in self.torques.clone() {
                body.add_torque(torque)
            }
        }

        for body in &mut self.bodies {
            body.update(dt)
        }

        for _ in 0..100 {
            self.check_collisions(debug);
        }
    }

    pub fn check_collisions(&mut self, debug: bool) {
        for body in &mut self.bodies {
            body.is_colliding = false;
        }

        for i in 0..self.bodies.len() {
            for j in (i + 1)..self.bodies.len() {
                if i != j {
                    // This is required to get past the borrow checker. Rust doesn't allow two mutable
                    // references to the vec. So self.bodies is split into two slices with split_at_mut.
                    // See markdown for more explanation.
                    let (left, right) = self.bodies.split_at_mut(i + 1);
                    let maybe_contact =
                        collision::is_colliding(&mut left[i], &mut right[j - i - 1]);

                    // is_colliding doesn't mutate a and b directly. But they need to be passed as mutable
                    // references since they will be used in instantiate a Contact class, which has mutable
                    // references to bodies. I'm not sure if this is the must idiomatic Rust way.

                    if let Some(mut contact) = maybe_contact {
                        contact.resolve_collision();

                        // if debug {
                        //     graphics::draw_fill_circle(
                        //         contact.start.x as i16,
                        //         contact.start.y as i16,
                        //         4.0 as i16,
                        //         0.,
                        //         0xFFFF00FF,
                        //     );
                        //     graphics::draw_fill_circle(
                        //         contact.end.x as i16,
                        //         contact.end.y as i16,
                        //         4.0 as i16,
                        //         0.,
                        //         0xFF00FF00,
                        //     );
                        //     graphics::draw_line(
                        //         contact.start.x as i16,
                        //         contact.start.y as i16,
                        //         (contact.start.x + contact.normal.x * 200.) as i16,
                        //         (contact.start.y + contact.normal.y * 200.) as i16,
                        //         0xFFFF00FF,
                        //     );
                        // }
                    }
                }
            }
        }
    }
}
