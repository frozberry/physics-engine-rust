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

    let ab = find_min_separation(a, b);
    let ba = find_min_separation(b, a);

    if ab >= 0. || ba >= 0. {
        return None;
    }

    // Temporary placeholder contact
    let v = Vec2::new(0., 0.);
    let contact = Contact::new(a, b, v, v, v, 0.);

    Some(contact)
}

pub fn is_collidng_circle_polygon<'a>(a: &'a mut Body, b: &'a mut Body) -> Option<Contact<'a>> {
    None
}

fn find_min_separation<'a>(a: &'a Body, b: &'a Body) -> f32 {
    let av = a.shape.get_world_verticies(a.rotation, a.pos);
    let bv = a.shape.get_world_verticies(b.rotation, b.pos);

    let mut separation = f32::MIN;

    for i in 0..av.len() {
        let va = av[i];
        let normal = a.shape.edge_at(i, a.rotation, a.pos).normal();

        let mut min_sep = f32::MAX;
        for j in 0..bv.len() {
            let vb = bv[j];
            min_sep = min_sep.min((vb - va).dot(normal));
        }
        separation = separation.max(min_sep)
    }
    separation
}
