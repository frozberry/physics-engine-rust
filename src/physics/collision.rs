use super::{body::Body, contact::Contact, shape::Shape, vec2::Vec2};

pub fn is_colliding<'a>(a: &'a Body, b: &'a Body) -> Option<Contact<'a>> {
    match a.shape {
        Shape::Circle(_) => match b.shape {
            Shape::Circle(_) => is_collidng_circle_circle(a, b),
            Shape::Polygon(_) => is_collidng_circle_polygon(a, b),
            Shape::Box(_, _) => is_collidng_circle_box(a, b),
        },
        Shape::Polygon(_) => match b.shape {
            Shape::Circle(_) => is_collidng_circle_polygon(a, b),
            Shape::Polygon(_) => is_collidng_polygon_polygon(a, b),
            Shape::Box(_, _) => is_collidng_polygon_box(a, b),
        },
        Shape::Box(_, _) => match b.shape {
            Shape::Circle(_) => is_collidng_circle_box(a, b),
            Shape::Polygon(_) => is_collidng_polygon_polygon(a, b),
            Shape::Box(_, _) => is_collidng_polygon_box(a, b),
        },
    }
}

pub fn is_collidng_circle_circle<'a>(a: &'a Body, b: &'a Body) -> Option<Contact<'a>> {
    let a_radius;
    let b_radius;
    match a.shape {
        Shape::Circle(a_r) => match b.shape {
            Shape::Circle(b_r) => {
                a_radius = a_r;
                b_radius = b_r;
            }
            _ => panic!("Non circle passed into collision function"),
        },
        _ => panic!("Non circle passed into collision function"),
    }

    let ab = b.pos - a.pos;
    let radius_sum = a_radius + b_radius;

    // Equivlent to: radius_sum >= ab.magnitude()
    let collision_detected = (radius_sum * radius_sum) >= ab.magnitude_squared();

    if collision_detected {
        let normal = ab.unit_vector();
        let start = b.pos - normal * b_radius;
        let end = a.pos + normal * a_radius;
        let depth = (end - start).magnitude();
        Some(Contact::new(a, b, start, end, normal, depth))
    } else {
        None
    }
}
pub fn is_collidng_circle_polygon<'a>(a: &'a Body, b: &'a Body) -> Option<Contact<'a>> {
    None
}
pub fn is_collidng_circle_box<'a>(a: &'a Body, b: &'a Body) -> Option<Contact<'a>> {
    None
}
pub fn is_collidng_polygon_polygon<'a>(a: &'a Body, b: &'a Body) -> Option<Contact<'a>> {
    None
}
pub fn is_collidng_box_box<'a>(a: &'a Body, b: &'a Body) -> Option<Contact<'a>> {
    None
}
pub fn is_collidng_polygon_box<'a>(a: &'a Body, b: &'a Body) -> Option<Contact<'a>> {
    None
}
