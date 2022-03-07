use super::object::Object;
use crate::util::{ray::Ray, vec::Vector3f};

pub struct Renderer {
    objs: Vec<Box<dyn Object>>,
    eye: Vector3f,
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {
            objs: Vec::new(),
            eye: Vector3f {x: 0.0, y: 1.0, z: 0.0},
        }
    }

    pub fn ray_march(&self, ray: &Ray) {
        
    }

    pub fn add_object(&mut self, obj: Box<dyn Object>) {
        self.objs.push(obj);
    }

    fn signed_dst_to_scene(&self) -> f32 {
        let mut dst_to_scene = f32::MAX;

        for i in &self.objs {
            let dst = i.signed_dst_from_point(&self.eye);
            dst_to_scene = dst_to_scene.max(dst);
        }

        dst_to_scene
    }
}
