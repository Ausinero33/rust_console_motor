use std::io::stdout;

use crossterm::{queue, style::{Color, SetForegroundColor, Print}};

use super::object::Object;
use crate::util::vec::Vector3f;

const MAX_STEPS: usize = 100;
const MAX_DISTANCE: f32 = 100.0;
const SURF_DISTANCE: f32 = 0.01;

pub struct Renderer {
    objs: Vec<Box<dyn Object>>,
    eye: Vector3f,
    light: Vector3f,
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {
            objs: Vec::new(),
            eye: Vector3f {x: 0.0, y: 1.0, z: 0.0},
            light: Vector3f {x: 0., y: 3., z: 6.},
        }
    }

    fn ray_march(&self, direction: Vector3f) -> f32 {
        let mut dst_from_origin = 0.0;

        for _ in 0..MAX_STEPS {
            let p = self.eye + direction * dst_from_origin;
            let dst_to_scene = self.signed_dst_to_scene(p);
            dst_from_origin += dst_to_scene;

            if dst_from_origin > MAX_DISTANCE || dst_to_scene < SURF_DISTANCE {
                break;
            }
        }

        return dst_from_origin;
    }

    pub fn add_object(&mut self, obj: Box<dyn Object>) {
        self.objs.push(obj);
    }

    fn get_normal(&self, point: Vector3f) -> Vector3f {
        let dst = self.signed_dst_to_scene(point);

        let n = Vector3f {
            x: dst - self.signed_dst_to_scene(point - Vector3f { x: SURF_DISTANCE, y: 0., z: 0. }),
            y: dst - self.signed_dst_to_scene(point - Vector3f { x: 0., y: SURF_DISTANCE, z: 0. }),
            z: dst - self.signed_dst_to_scene(point - Vector3f { x: 0., y: 0., z: SURF_DISTANCE }),
        };

        n.normalize()
    }

    fn get_light(&self, point: Vector3f) -> f32 {
        let light = (self.light - point).normalize();
        let normal = self.get_normal(point);

        normal.dot_product(light)
    }

    pub fn render(&self,  size: &(u16, u16)) {
        for row in 0..size.1 {
            for col in 0..size.0 {
                let dir = Vector3f {
                    x: (col as f32 - (size.0 / 2) as f32) / 2.,
                    y: -(row as f32 - (size.1 / 2) as f32),
                    // Aqui igual hay que variarlo según el FOV
                    z: 20.,
                };

                let dst = self.ray_march(dir.normalize());
                
                let p = self.eye + dir.normalize() * dst;
                let light = (self.get_light(p) * 255.) as u8;
                // let normal = self.get_normal(p) * 255.0;
        
                // let color = Color::Rgb{ r: normal.x as u8, g: normal.y as u8, b: normal.z as u8 };
                let color = Color::Rgb{ r: light, g: light, b: light };
                queue!{
                    stdout(),
                    SetForegroundColor(color),
                    Print("\u{2588}"),
                }.unwrap();
            } 
        }
    }

    fn signed_dst_to_scene(&self, point: Vector3f) -> f32 {
        let mut dst_to_scene = f32::MAX; 

        for i in &self.objs {
            let dst = i.signed_dst_from_point(point);
            dst_to_scene = dst_to_scene.min(dst);

        }

        dst_to_scene
    }
}
