use super::vec2::Vec2;

#[derive(Clone, Debug)]
pub enum Shape {
    // Circle(radius)
    Circle(f32),

    // Polygon(Vec<vertices>)
    Polygon(Vec<Vec2>),

    // Box<width, height, x, y>
    Box(f32, f32),
}

impl Shape {
    pub fn calc_inertia(&self, mass: f32) -> f32 {
        match self {
            // 1/2 * m   r^2
            Shape::Circle(radius) => mass * radius * radius * 0.5,
            // 1/12 * (w^2 + h ^2)
            Shape::Box(w, h) => mass * 0.0833333 * (w * w + h * h),
            _ => 0.,
        }
    }

    pub fn get_local_verticies(&self) -> Option<Vec<Vec2>> {
        match self {
            Shape::Circle(_) => None,
            Shape::Polygon(vertices) => Some(vertices.to_vec()),
            Shape::Box(w, h) => {
                let a = Vec2::new(-w / 2., -h / 2.);
                let b = Vec2::new(w / 2., -h / 2.);
                let c = Vec2::new(w / 2., h / 2.);
                let d = Vec2::new(-w / 2., h / 2.);
                Some(vec![a, b, c, d])
            }
        }
    }

    pub fn get_world_verticies(&self, rotation: f32, pos: Vec2) -> Option<Vec<Vec2>> {
        match self {
            Shape::Circle(_) => None,
            Shape::Polygon(vertices) => Some(vertices.to_vec()),
            Shape::Box(_, _) => Some(
                self.get_local_verticies()
                    .unwrap()
                    .iter()
                    .map(|vertex| vertex.rotate(rotation) + pos)
                    .collect(),
            ),
        }
    }

    pub fn edge_at(&self, index: usize, rotation: f32, pos: Vec2) -> Vec2 {
        match self {
            Shape::Circle(_) => {
                panic!("Edge at called with Circle shape")
            }
            Shape::Polygon(vertices) => {
                let current_i = index;
                let next_i = (index + 1) % vertices.len();
                vertices[next_i] - vertices[current_i]
            }
            Shape::Box(_, _) => {
                let vertices = self.get_world_verticies(rotation, pos).unwrap();
                let current_i = index;
                let next_i = (index + 1) % vertices.len();
                vertices[next_i] - vertices[current_i]
            }
        }
    }
}
