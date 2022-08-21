use core::panic;

use crate::graphics;

use super::{body::Body, contact::Contact, shape::Shape, vec2::Vec2};

pub fn is_colliding<'a>(a: &'a mut Body, b: &'a mut Body) -> Option<Contact<'a>> {
    match a.shape {
        Shape::Circle(_) => match b.shape {
            Shape::Circle(_) => is_collidng_circle_circle(a, b),
            Shape::Polygon(_) => is_collidng_circle_polygon(a, b),
            Shape::Box(_, _) => is_collidng_circle_polygon(a, b),
        },
        Shape::Polygon(_) => match b.shape {
            Shape::Circle(_) => is_collidng_circle_polygon(b, a),
            Shape::Polygon(_) => is_collidng_polygon_polygon(a, b),
            Shape::Box(_, _) => is_collidng_polygon_polygon(b, a),
        },
        Shape::Box(_, _) => match b.shape {
            Shape::Circle(_) => is_collidng_circle_polygon(b, a),
            Shape::Polygon(_) => is_collidng_polygon_polygon(a, b),
            Shape::Box(_, _) => is_collidng_polygon_polygon(b, a),
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

pub fn is_collidng_circle_polygon<'a>(
    circle: &'a mut Body,
    polygon: &'a mut Body,
) -> Option<Contact<'a>> {
    // TODO writeup this design choice that enum needs to panic
    if let Shape::Circle(_) = polygon.shape {
        panic!("Incorrect shape");
    }
    if let Shape::Box(_, _) = circle.shape {
        panic!("Incorrect shape");
    }
    if let Shape::Polygon(_) = circle.shape {
        panic!("Incorrect shape");
    }

    let mut radius = 0.;
    if let Shape::Circle(r) = circle.shape {
        radius = r;
    }

    let verticies = polygon
        .shape
        .get_world_verticies(polygon.rotation, polygon.pos);

    let mut is_outside = false;
    let mut min_curr_vertex = Vec2::new(0., 0.);
    let mut min_next_vertex = Vec2::new(0., 0.);
    let mut distance_circle_edge = f32::MIN;

    for i in 0..verticies.len() {
        let current_vertex = i;
        let next_vertex = (i + 1) % verticies.len();
        // TODO writeup this design choice to pass in pos and rot
        let edge = polygon
            .shape
            .edge_at(current_vertex, polygon.rotation, polygon.pos);

        let normal = edge.normal();

        let vertex_to_circle_center = circle.pos - verticies[current_vertex];
        let projection = vertex_to_circle_center.dot(normal);

        // If projection is positive/outsid the normal
        if projection > 0. {
            is_outside = true;
            distance_circle_edge = projection;
            min_curr_vertex = verticies[current_vertex];
            min_next_vertex = verticies[next_vertex];
            break;
        } else {
            // Circle is insid ethe polygon, find the min edge (least negative proejction)
            if projection > distance_circle_edge {
                distance_circle_edge = projection;
                min_curr_vertex = verticies[current_vertex];
                min_next_vertex = verticies[next_vertex];
            }
        }
    }

    let a = min_curr_vertex;
    let b = min_next_vertex;

    let ab = b - a;
    let ac = circle.pos - a;
    let bc = circle.pos - b;
    if is_outside {
        // Circle is in region A
        if ac.dot(ab) < 0. {
            if radius < ac.magnitude() {
                return None;
            }

            let start = circle.pos - ac.unit_vector() * radius;
            let end = a;
            let normal = ac.unit_vector();
            let depth = (end - start).magnitude();

            return Some(Contact::new(circle, polygon, start, end, normal, depth));
        }

        // Circle is in region B
        if bc.dot(ab) > 0. {
            if radius < bc.magnitude() {
                return None;
            }

            let start = circle.pos - bc.unit_vector() * radius;
            let end = b;
            let normal = bc.unit_vector();
            let depth = (end - start).magnitude();

            return Some(Contact::new(circle, polygon, start, end, normal, depth));
        }

        // Circle is in region C
        if radius < distance_circle_edge {
            return None;
        }

        let depth = radius - distance_circle_edge;
        let normal = ab.normal();
        let start = circle.pos - normal * radius;
        let end = start + normal * depth;

        return Some(Contact::new(circle, polygon, start, end, normal, depth));
    }

    // If center of circle is inside the polygon
    let depth = radius - distance_circle_edge;
    let normal = ab.normal();
    let start = circle.pos - normal * radius;
    let end = start + normal * depth;

    Some(Contact::new(circle, polygon, start, end, normal, depth))
}
