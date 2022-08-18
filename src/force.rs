use crate::physics::{particle::Particle, vec2::Vec2};

// F = k * |v|^2 * -(unit_vector)
pub fn generate_drag_force(particle: &Particle, k: f32) -> Vec2 {
    let mut drag_force = Vec2::new(0., 0.);

    // Skip calculation if unecessary
    if particle.vel.magnitude_squared() > 0. {
        let direction = particle.vel.normalized() * -1.;
        let magnitude = k * particle.vel.magnitude_squared();
        drag_force = direction * magnitude;
    }
    drag_force
}

pub fn generate_friction_force(particle: &Particle, k: f32) -> Vec2 {
    let direction = particle.vel.normalized() * -1.;
    let magnitude = k * particle.vel.magnitude_squared();

    direction * magnitude
}

// F = (G * m_a * m_b / d^2) * unit vector
pub fn generate_gravitational_force(
    a: &Particle,
    b: &Particle,
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
