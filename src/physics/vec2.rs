use std::ops;

#[derive(PartialEq, Debug, Clone, Copy)]

pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Vec2 { x, y }
    }

    pub fn add(&mut self, vec: Vec2) {
        self.x += vec.x;
        self.y += vec.y
    }

    pub fn subtract(&mut self, vec: Vec2) {
        self.x -= vec.x;
        self.y -= vec.y
    }

    pub fn scale(&mut self, n: f32) {
        self.x *= n;
        self.y *= n;
    }

    pub fn rotate(&mut self, angle: f32) {
        self.x = self.x * angle.cos() - self.y * angle.sin();
        self.y = self.x * angle.sin() + self.y * angle.cos();
    }

    pub fn magnitude(&self) -> f32 {
        (f32::powf(self.x, 2.) + f32::powf(self.y, 2.)).sqrt()
    }

    pub fn magnitude_squared(&self) -> f32 {
        f32::powf(self.x, 2.) + f32::powf(self.y, 2.)
    }

    // C++ returns an pointer to Vec2, my code just mutates in place
    pub fn normalize(&mut self) {
        let length = self.magnitude();

        if length != 0. {
            self.x /= length;
            self.y /= length;
        }
    }
    pub fn normalized(&self) -> Vec2 {
        let length = self.magnitude();

        let x = if self.x != 0. { self.x / length } else { 0. };
        let y = if self.y != 0. { self.y / length } else { 0. };

        Vec2::new(x, y)
    }

    pub fn unit_vector(&self) -> Vec2 {
        let mut result = Vec2::new(0., 0.);
        let length = self.magnitude();
        if length != 0. {
            result.x = self.x / length;
            result.y = self.y / length;
        }
        result
    }

    // This is coded differently since normalize() doesn't return anything
    pub fn normal(&self) -> Vec2 {
        let mut perpendicular = Vec2::new(self.y, -self.x);
        perpendicular.normalize();
        perpendicular
    }

    pub fn dot(&self, v: Vec2) -> f32 {
        self.x * v.x + self.y * v.y
    }

    pub fn cross(&self, v: Vec2) -> f32 {
        self.x * v.y - self.y * v.x
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

// Scale
impl ops::Mul<f32> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec2::new(self.x * rhs, self.y * rhs)
    }
}

// Dot product
impl ops::Mul<Vec2> for Vec2 {
    type Output = f32;

    fn mul(self, rhs: Vec2) -> Self::Output {
        self.x * rhs.x + self.y + rhs.y
    }
}

impl ops::Div<f32> for Vec2 {
    type Output = Vec2;

    fn div(self, rhs: f32) -> Self::Output {
        Vec2::new(self.x / rhs, self.y / rhs)
    }
}

impl ops::AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Vec2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl ops::SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Vec2) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl ops::SubAssign<f32> for Vec2 {
    fn sub_assign(&mut self, rhs: f32) {
        self.x -= rhs;
        self.y -= rhs;
    }
}

impl ops::MulAssign<f32> for Vec2 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl ops::DivAssign<f32> for Vec2 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl ops::Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec2::new(self.x * -1., self.y * -1.)
    }
}

// #[cfg(test)]
// mod tests {
//     use std::f32::consts::PI;

//     use super::*;
//     use assert_approx_eq::assert_approx_eq;

//     #[test]
//     fn test_magnitude() {
//         let v1 = Vec2::new(3., -4.);
//         let v2 = Vec2::new(1.2, 2.1);
//         assert_eq!(v1.magnitude(), 5.);
//         assert_approx_eq!(v2.magnitude(), 2.418677);
//     }

//     #[test]
//     fn test_add() {
//         let v1 = Vec2::new(1., 2.);
//         let v2 = Vec2::new(3., 4.);
//         let result = Vec2::new(4., 6.);
//         assert_eq!(v1 + v2, result);
//     }

//     #[test]
//     fn test_subtract() {
//         let v1 = Vec2::new(1., 2.);
//         let v2 = Vec2::new(3., 4.);
//         let result = Vec2::new(-2., -2.);
//         assert_eq!(v1 - v2, result);
//     }
//     #[test]
//     fn test_dot_product() {
//         todo!()
//     }

//     #[test]
//     fn test_multiply_scalar() {
//         let v1 = Vec2::new(1.5, -2.);
//         let result = Vec2::new(3., -4.);
//         assert_eq!(v1 * 2., result);
//     }

//     #[test]
//     fn test_perpendicular() {
//         todo!()
//     }

//     #[test]
//     fn test_normalize() {
//         let mut v1 = Vec2::new(234.4, -34.5);
//         v1.normalize();
//         assert_eq!(v1.magnitude(), 1.)
//     }

//     // #[test]
//     // fn test_rotate() {
//     //     let v1 = Vec2::new(2., 4.);
//     //     let result = Vec2::new(2., 4.);
//     //     assert_eq!(v1.rotate(2. * PI), result)
//     // }
// }
