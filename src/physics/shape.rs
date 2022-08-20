use super::vec2::Vec2;

trait Foo {
    fn foo(&self) -> i32;
}

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
            Shape::Polygon(_) => panic!("Polygon inertia unimplemented"),
        }
    }

    pub fn get_local_verticies(&self) -> Vec<Vec2> {
        match self {
            Shape::Circle(_) => panic!("Circle has no verticies"),
            Shape::Polygon(vertices) => vertices.to_vec(),
            Shape::Box(w, h) => {
                let a = Vec2::new(-w / 2., -h / 2.);
                let b = Vec2::new(w / 2., -h / 2.);
                let c = Vec2::new(w / 2., h / 2.);
                let d = Vec2::new(-w / 2., h / 2.);
                vec![a, b, c, d]
            }
        }
    }

    pub fn get_world_verticies(&self, rotation: f32, pos: Vec2) -> Vec<Vec2> {
        let local_to_world = || {
            self.get_local_verticies()
                .iter()
                .map(|vertex| vertex.rotate(rotation) + pos)
                .collect()
        };

        match self {
            Shape::Circle(_) => panic!("Circle has no vertices"),
            Shape::Polygon(_) => local_to_world(),
            Shape::Box(_, _) => local_to_world(),
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
                let vertices = self.get_world_verticies(rotation, pos);
                let current_i = index;
                let next_i = (index + 1) % vertices.len();
                vertices[next_i] - vertices[current_i]
            }
        }
    }
}
