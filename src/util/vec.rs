use std::ops::{Add, Sub};

pub struct Vector3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3f {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3f {
        Vector3f {
            x,
            y,
            z,
        }
    }

    pub fn dot_product(&self, v: Vector3f) -> f32 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    pub fn length(&self) -> f32 {
        f32::sqrt(f32::powi(self.x, 2) + f32::powi(self.y, 2) + f32::powi(self.z, 2))
    }

    pub fn max(&self) -> f32 {
        // Ekisde
        return self.x.max(self.y.max(self.z));
    }

    pub fn abs(&self) -> Vector3f {
        Vector3f {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }
}

impl Add for Vector3f {
    type Output = Vector3f;

    fn add(self, rhs: Vector3f) -> Vector3f {
        Vector3f {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vector3f {
    type Output = Vector3f;

    fn sub(self, rhs: Vector3f) -> Vector3f {
        Vector3f {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
