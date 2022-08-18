use super::vec2::Vec2;

pub enum Shape {
    // Circle(radius)
    Circle(f32),

    // Polygon(Vec<vertices>)
    Polygon(Vec<Vec2>),

    // Box<width, height, x, y>
    Box(f32, f32),
}
