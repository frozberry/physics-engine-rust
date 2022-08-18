use super::{body::Body, shape::Shape};

pub fn is_colliding(a: &Body, b: &Body) -> bool {
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

pub fn is_collidng_circle_circle(a: &Body, b: &Body) -> bool {
    let ab = a.pos - b.pos;
    let radius_sum;

    match a.shape {
        Shape::Circle(a_r) => match b.shape {
            Shape::Circle(b_r) => radius_sum = a_r + b_r,
            _ => panic!("Non circle passed into collision function"),
        },
        _ => panic!("Non circle passed into collision function"),
    }

    // Equivlent to: radius_sum >= ab.magnitude()
    (radius_sum * radius_sum) >= ab.magnitude_squared()
}

pub fn is_collidng_circle_polygon(a: &Body, b: &Body) -> bool {
    false
}
pub fn is_collidng_circle_box(a: &Body, b: &Body) -> bool {
    false
}
pub fn is_collidng_polygon_polygon(a: &Body, b: &Body) -> bool {
    false
}
pub fn is_collidng_box_box(a: &Body, b: &Body) -> bool {
    false
}
pub fn is_collidng_polygon_box(a: &Body, b: &Body) -> bool {
    false
}
