use std::ops::{Add, Sub, Mul};


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
        f32::sqrt(f32::powi(self.x, 2) + f32::powi(self.y, 2) + f32::powi(self.z, 2))
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
        let length = self.length();

        Vector3f {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
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
