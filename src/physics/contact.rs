use super::{body::Body, vec2::Vec2};

pub struct Contact<'a> {
    pub a: &'a Body,
    pub b: &'a Body,
    pub start: Vec2,
    pub end: Vec2,
    pub normal: Vec2,
    pub depth: f32,
}

impl<'a> Contact<'a> {
    pub fn new(a: &'a Body, b: &'a Body, start: Vec2, end: Vec2, normal: Vec2, depth: f32) -> Self {
        Contact {
            a,
            b,
            start,
            end,
            normal,
            depth,
        }
    }
}
