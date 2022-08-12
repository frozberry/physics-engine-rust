use std::ops;

#[derive(PartialEq, Debug)]

struct Vec2 {
    x: f32,
    y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Vec2 { x, y }
    }

    pub fn magnitude(&self) -> f32 {
        (f32::powf(self.x, 2.) + f32::powf(self.y, 2.)).sqrt()
    }
}

impl ops::Add for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Self) -> Self::Output {
        Vec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl ops::Sub for Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_magnitude() {
        let v1 = Vec2::new(3., -4.);
        let v2 = Vec2::new(1.2, 2.1);
        assert_eq!(v1.magnitude(), 5.);
        assert_approx_eq!(v2.magnitude(), 2.418677);
    }

    #[test]
    fn test_add() {
        let v1 = Vec2::new(1., 2.);
        let v2 = Vec2::new(3., 4.);
        let result = Vec2::new(4., 6.);
        assert_eq!(v1 + v2, result);
    }

    #[test]
    fn test_subtract() {
        let v1 = Vec2::new(1., 2.);
        let v2 = Vec2::new(3., 4.);
        let result = Vec2::new(-2., -2.);
        assert_eq!(v1 - v2, result);
    }
}
