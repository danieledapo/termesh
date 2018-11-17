//! A simple Vector3

use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Sub};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vector3 { x, y, z }
    }

    pub fn round(&self) -> Self {
        Vector3::new(self.x.round(), self.y.round(), self.z.round())
    }

    /// Rotate the point around the X axis by the given angle in radians
    pub fn rotate_x(&mut self, angle: f32) {
        let cosa = angle.cos();
        let sina = angle.sin();

        let y = self.y * cosa - self.z * sina;
        let z = self.y * sina + self.z * cosa;

        self.y = y;
        self.z = z;
    }

    /// Rotate the point around the Y axis by the given angle in radians
    pub fn rotate_y(&mut self, angle: f32) {
        let cosa = angle.cos();
        let sina = angle.sin();

        let z = self.z * cosa - self.x * sina;
        let x = self.z * sina + self.x * cosa;

        self.z = z;
        self.x = x;
    }

    /// Rotate the point around the Z axis by the given angle in radians
    pub fn rotate_z(&mut self, angle: f32) {
        let cosa = angle.cos();
        let sina = angle.sin();

        let x = self.x * cosa - self.y * sina;
        let y = self.x * sina + self.y * cosa;

        self.x = x;
        self.y = y;
    }
}

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Vector3 {
        Vector3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, other: Vector3) -> Vector3 {
        Vector3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, f: f32) -> Vector3 {
        Vector3::new(self.x * f, self.y * f, self.z * f)
    }
}

impl Div<f32> for Vector3 {
    type Output = Vector3;

    fn div(self, d: f32) -> Vector3 {
        Vector3::new(self.x / d, self.y / d, self.z / d)
    }
}

impl AddAssign for Vector3 {
    fn add_assign(&mut self, other: Vector3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl MulAssign<f32> for Vector3 {
    fn mul_assign(&mut self, f: f32) {
        self.x *= f;
        self.y *= f;
        self.z *= f;
    }
}
