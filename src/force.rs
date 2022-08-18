use crate::physics::{body::Body, vec2::Vec2};

// F = k * |v|^2 * -(unit_vector)
pub fn generate_drag_force(particle: &Body, k: f32) -> Vec2 {
    let mut drag_force = Vec2::new(0., 0.);

    // Skip calculation if unecessary
    if particle.vel.magnitude_squared() > 0. {
        let direction = particle.vel.normalized() * -1.;
        let magnitude = k * particle.vel.magnitude_squared();
        drag_force = direction * magnitude;
    }
    drag_force
}

pub fn generate_friction_force(particle: &Body, k: f32) -> Vec2 {
    let direction = particle.vel.normalized() * -1.;
    let magnitude = k * particle.vel.magnitude_squared();

    direction * magnitude
}

// F = (G * m_a * m_b / d^2) * unit vector
pub fn generate_gravitational_force(
    a: &Body,
    b: &Body,
    g: f32,
    min_distance: f32,
    max_distance: f32,
) -> Vec2 {
    let distance = b.pos - a.pos;
    let distance_squared = distance
        .magnitude_squared()
        .clamp(min_distance, max_distance);
    let magnitude = (g * a.mass * b.mass) / distance_squared;

    distance * magnitude
}

pub fn generate_spring_force(particle: &Body, anchor: Vec2, rest_length: f32, k: f32) -> Vec2 {
    let distance = particle.pos - anchor;
    let displacement = distance.magnitude() - rest_length;

    let direction = distance.unit_vector();
    let magnitude = -k * displacement;

    direction * magnitude
}

pub fn generate_spring_force_particles(a: &Body, b: &Body, rest_length: f32, k: f32) -> Vec2 {
    let distance = a.pos - b.pos;
    let displacement = distance.magnitude() - rest_length;

    let direction = distance.unit_vector();
    let magnitude = -k * displacement;

    direction * magnitude
}
