use std::ops;

#[derive(PartialEq, Debug)]

struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z }
    }

    pub fn magnitude(&self) -> f32 {
        (f32::powf(self.x, 2.) + f32::powf(self.y, 2.) + f32::powf(self.z, 2.)).sqrt()
    }

    pub fn dot(&self, b: Vec3) -> f32 {
        self.x * b.x + self.y + b.y + self.z * b.z
    }

    pub fn cross(&self, b: Vec3) -> Vec3 {
        let x = self.y * b.z - self.z * b.y;
        let y = self.z * b.x - self.x * b.z;
        let z = self.x * b.y - self.y * b.x;
        Vec3::new(x, y, z)
    }

    pub fn normalize(&self) -> Vec3 {
        let length = &self.magnitude();
        Vec3::new(self.x / length, self.y / length, self.z / length)
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

// Dot product
impl ops::Mul<Vec3> for Vec3 {
    type Output = f32;

    fn mul(self, rhs: Vec3) -> Self::Output {
        self.x * rhs.x + self.y + rhs.y + self.z * rhs.z
    }
}

// Scale
impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use assert_approx_eq::assert_approx_eq;

//     #[test]
//     fn test_magnitude() {
//         let v1 = Vec3::new(3., -4.);
//         let v2 = Vec3::new(1.2, 2.1);
//         assert_eq!(v1.magnitude(), 5.);
//         assert_approx_eq!(v2.magnitude(), 2.418677);
//     }

//     #[test]
//     fn test_add() {
//         let v1 = Vec3::new(1., 2.);
//         let v2 = Vec3::new(3., 4.);
//         let result = Vec3::new(4., 6.);
//         assert_eq!(v1 + v2, result);
//     }

//     #[test]
//     fn test_subtract() {
//         let v1 = Vec3::new(1., 2.);
//         let v2 = Vec3::new(3., 4.);
//         let result = Vec3::new(-2., -2.);
//         assert_eq!(v1 - v2, result);
//     }
//     #[test]
//     fn test_dot_product() {
//         todo!()
//     }

//     #[test]
//     fn test_multiply_scalar() {
//         let v1 = Vec3::new(1.5, -2.);
//         let result = Vec3::new(3., -4.);
//         assert_eq!(v1 * 2., result);
//     }

//     #[test]
//     fn test_perpendicular() {
//         todo!()
//     }
// }
