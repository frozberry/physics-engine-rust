use super::{body::Body, contact::Contact, shape::Shape, vec2::Vec2};

pub fn is_colliding<'a>(a: &'a mut Body, b: &'a mut Body) -> Option<Contact<'a>> {
    match a.shape {
        Shape::Circle(_) => match b.shape {
            Shape::Circle(_) => is_collidng_circle_circle(a, b),
            Shape::Polygon(_) => is_collidng_circle_polygon(a, b),
            Shape::Box(_, _) => is_collidng_circle_polygon(a, b),
        },
        Shape::Polygon(_) => match b.shape {
            Shape::Circle(_) => is_collidng_circle_polygon(a, b),
            Shape::Polygon(_) => is_collidng_polygon_polygon(a, b),
            Shape::Box(_, _) => is_collidng_polygon_polygon(a, b),
        },
        Shape::Box(_, _) => match b.shape {
            Shape::Circle(_) => is_collidng_circle_polygon(a, b),
            Shape::Polygon(_) => is_collidng_polygon_polygon(a, b),
            Shape::Box(_, _) => is_collidng_polygon_polygon(a, b),
        },
    }
}

pub fn is_collidng_circle_circle<'a>(a: &'a mut Body, b: &'a mut Body) -> Option<Contact<'a>> {
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

pub fn is_collidng_polygon_polygon<'a>(a: &'a mut Body, b: &'a mut Body) -> Option<Contact<'a>> {
    if let Shape::Circle(_) = a.shape {
        panic!("Wrong collision function called")
    }
    if let Shape::Circle(_) = b.shape {
        panic!("Wrong collision function called")
    }

    let (ab_seperation, a_axis, a_point) = find_min_separation(a, b);
    let (ba_seperation, b_axis, b_point) = find_min_separation(b, a);

    if ab_seperation >= 0. || ba_seperation >= 0. {
        return None;
    }

    if ab_seperation > ba_seperation {
        let depth = -ab_seperation;
        let normal = a_axis.normal();
        let start = a_point;
        let end = a_point + normal * depth;
        Some(Contact::new(a, b, start, end, normal, depth))
    } else {
        let depth = -ba_seperation;
        let normal = -b_axis.normal();
        let start = b_point - normal * depth;
        let end = b_point;
        Some(Contact::new(a, b, start, end, normal, depth))
    }
}

pub fn is_collidng_circle_polygon<'a>(a: &'a mut Body, b: &'a mut Body) -> Option<Contact<'a>> {
    None
}

fn find_min_separation<'a>(a: &'a Body, b: &'a Body) -> (f32, Vec2, Vec2) {
    let a_vertices = a.shape.get_world_verticies(a.rotation, a.pos);
    let b_vertices = b.shape.get_world_verticies(b.rotation, b.pos);

    let mut separation = f32::MIN;
    let mut axis = Vec2::new(0., 0.);
    let mut point = Vec2::new(0., 0.);

    for i in 0..a_vertices.len() {
        let va = a_vertices[i];
        let normal = a.shape.edge_at(i, a.rotation, a.pos).normal();

        let mut min_sep = f32::MAX;
        let mut min_vertex = Vec2::new(0., 0.);

        for j in 0..b_vertices.len() {
            let vb = b_vertices[j];
            let proj = (vb - va).dot(normal);
            if proj < min_sep {
                min_sep = proj;
                min_vertex = vb;
            }
        }
        if min_sep > separation {
            separation = min_sep;
            axis = a.shape.edge_at(i, a.rotation, a.pos);
            point = min_vertex;
        }
    }
    (separation, axis, point)
}
