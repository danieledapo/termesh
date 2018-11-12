//! A simple Vector3

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

    pub fn scale(&mut self, factor: f32) {
        self.x *= factor;
        self.y *= factor;
        self.z *= factor;
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
