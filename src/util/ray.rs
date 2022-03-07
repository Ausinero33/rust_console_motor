use super::vec::Vector3f;

pub struct Ray {
    origin: Vector3f,
    direction: Vector3f,
}

impl Ray {
    pub fn new(origin: Vector3f, direction: Vector3f) -> Ray {
        let dir = direction.normalize();
    }
    
    pub fn march(self, dst: f32) -> Ray {
        Ray {
            origin: self.origin + self.direction * dst,
            direction: self.direction,
            
        }
    }
}
