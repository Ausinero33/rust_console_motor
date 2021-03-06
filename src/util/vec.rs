use core::fmt;
use std::ops::{Add, Sub, Mul, Div};


#[derive(Clone, Copy)]
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
        // f32::sqrt(f32::powi(self.x, 2) + f32::powi(self.y, 2) + f32::powi(self.z, 2))
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn max(&self) -> f32 {
        // Ekisde
        return self.x.max(self.y.max(self.z));
    }

    pub fn max_vec(&self, num: f32) -> Vector3f {
        Vector3f {
            x: self.x.max(num),
            y: self.y.max(num),
            z: self.z.max(num),
        }
    }

    pub fn abs(&self) -> Vector3f {
        Vector3f {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs()
        }
    }

    pub fn normalize(self) -> Vector3f {
        self / self.length()
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

impl Mul<f32> for Vector3f {
    type Output = Vector3f;

    fn mul(self, rhs: f32) -> Vector3f {
        Vector3f {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,

        }
    }
}

impl Div<f32> for Vector3f {
    type Output = Vector3f;

    fn div(self, rhs: f32) -> Self::Output {
        Vector3f {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl fmt::Debug for Vector3f {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Vector3f")
            .field("x", &self.x)
            .field("y", &self.y)
            .field("z", &self.z)
            .finish()
    }
}
