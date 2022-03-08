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
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {
            objs: Vec::new(),
            eye: Vector3f {x: 0.0, y: 1.0, z: 0.0},
        }
    }

    pub fn ray_march(&self, direction: Vector3f) -> f32 {
        let mut dst_from_origin = 0.0;

        'step: for _ in 0..MAX_STEPS {
            let p = self.eye + direction * dst_from_origin;
            let dst_to_scene = self.signed_dst_to_scene(p);
            dst_from_origin += dst_to_scene;

            if dst_from_origin > MAX_DISTANCE || dst_to_scene < SURF_DISTANCE {
                break 'step;
            }
        }

        return dst_from_origin;
    }

    pub fn add_object(&mut self, obj: Box<dyn Object>) {
        self.objs.push(obj);
    }

    pub fn render(&self, buffer: &mut [u8], size: &(u16, u16)) {
        for row in 0..size.1 {
            for col in 0..size.0 {
                let dir = Vector3f {
                    x: row as f32 - (size.1 / 2) as f32,
                    y: (col as f32 - (size.0 / 2) as f32) / 2.0,
                    // Aqui igual hay que variarlo según el FOV
                    z: 4.5,
                }.normalize();

                let dst = self.ray_march(dir);

                let color = if dst < MAX_DISTANCE {
                    // 0x30
                    Color::White
                } else {
                    // 0x20
                    Color::Black
                };

                queue!{
                    stdout(),
                    SetForegroundColor(color),
                    Print("\u{2588}"),
                }.unwrap();
                // buffer[(col + row * size.0) as usize] = dst as u8 + 0x20;
                // buffer[(col + row * size.0) as usize] = (col + row * size.0) as u8;
                // buffer[(col + row * size.0) as usize] = char;
            } 
        }
    }

    fn signed_dst_to_scene(&self, point: Vector3f) -> f32 {
        let mut dst_to_scene = MAX_DISTANCE;

        for i in &self.objs {
            let dst = i.signed_dst_from_point(point);
            dst_to_scene = dst_to_scene.min(dst);

        }

        dst_to_scene
    }
}
