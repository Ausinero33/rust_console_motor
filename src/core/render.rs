use std::io::stdout;

use crossterm::{queue, style::{Color, SetForegroundColor, Print}};

use super::object::Object;
use crate::util::vec::Vector3f;

const MAX_STEPS: usize = 100;
const MAX_DISTANCE: f32 = 100.0;
const SURF_DISTANCE: f32 = 0.01;

pub struct Renderer {
    objs: Vec<Box<dyn Object>>,
    pub eye: Vector3f,
    pub light: Vector3f,
    pub initial_light: Vector3f,
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {
            objs: Vec::new(),
            eye: Vector3f {x: 0., y: 0., z: -3.},
            light: Vector3f {x: 0., y: 0., z: -1.5},
            initial_light: Vector3f { x: 0., y: 3., z: 6. },
        }
    }

    fn ray_march(&self, origin: Vector3f, direction: Vector3f) -> f32 {
        let mut dst_from_origin = 0.0;

        for _ in 0..MAX_STEPS {
            let p = origin + direction * dst_from_origin;
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
        let x_pl = self.signed_dst_to_scene( Vector3f { x: point.x + 0.001, y: point.y, z: point.z } );
        let x_mi = self.signed_dst_to_scene( Vector3f { x: point.x - 0.001, y: point.y, z: point.z } );
        let y_pl = self.signed_dst_to_scene( Vector3f { x: point.x, y: point.y + 0.001, z: point.z } );
        let y_mi = self.signed_dst_to_scene( Vector3f { x: point.x, y: point.y - 0.001, z: point.z } );
        let z_pl = self.signed_dst_to_scene( Vector3f { x: point.x, y: point.y, z: point.z + 0.001 } );
        let z_mi = self.signed_dst_to_scene( Vector3f { x: point.x, y: point.y, z: point.z - 0.001 } );

        let x_dif = x_pl - x_mi;
        let y_dif = y_pl - y_mi;
        let z_dif = z_pl - z_mi;

        Vector3f { x: x_dif, y: y_dif, z: z_dif }.normalize()
    }

    fn get_light(&self, point: Vector3f) -> f32 {
        let light = (self.light - point).normalize();
        let normal =  self.get_normal(point);
 
        let d = self.ray_march(point + normal * SURF_DISTANCE * 2., light);
        let mut dif = normal.dot_product(light);
        
        if dif < 0. {
            dif = 0.;
        }

        if d < (self.light - point).length() {
            dif *= 0.1;
        }

        dif
    }

    pub fn render(&self,  size: &(u16, u16)) {
        for row in 0..size.1 {
            for col in 0..size.0 {
                let dir = Vector3f {
                    x: (col as f32 - (size.0 / 2) as f32) / 2.,
                    y: -(row as f32 - (size.1 / 2) as f32),
                    // Aqui igual hay que variarlo según el FOV
                    z: 30.,
                }.normalize();

                let dst = self.ray_march(self.eye, dir);
                
                let p = self.eye + dir * dst;
                let light = (self.get_light(p) * 255.) as u8;
                let normal = self.get_normal(p) * 255.0;
        
                // let color = Color::Rgb{ r: dst as u8, g: dst as u8, b: dst as u8 };
                let color = Color::Rgb{ r: normal.x as u8, g: normal.y as u8, b: normal.z as u8 };
                // let color = Color::Rgb{ r: light, g: light, b: light };
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

        self.objs.iter()
            .for_each(|x| dst_to_scene = x.signed_dst_from_point(point).min(dst_to_scene));

        dst_to_scene
    }
}
