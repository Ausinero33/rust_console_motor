use std::{io::{stdout, Write}, time::{Instant, Duration}, thread, ops::Add};

use crossterm::{execute, terminal::{size, self}, cursor, queue, style};

use crate::{core::object::Object, util::vec::Vector3f};

pub struct App {
    framerate: u16,
    frame_buffer: Vec<u8>,
    objs: Vec<Box<dyn Object>>,
    eye: Vector3f,
}

impl App {
    pub fn new() -> App {
        let size = size().unwrap();

        execute!(
            stdout(), 
            terminal::EnterAlternateScreen,
            terminal::EnableLineWrap,
            cursor::Hide,
        ).unwrap();
        

        App {
            framerate: 24,
            frame_buffer: vec![0x24; (size.0 * size.1) as usize],
            objs: Vec::new(),
            eye: Vector3f{x: 0f32, y: 0f32, z: 0f32},
        }
    }

    pub fn set_framerate(mut self, framerate: u16) -> App {
        self.framerate = framerate;
        self
    }

    pub fn run(&self) {
        loop {
            let start = Instant::now();

            queue!(
                stdout(),
                cursor::MoveTo(0, 0),
                style::Print(String::from_utf8_lossy(self.frame_buffer.as_slice())),
            ).unwrap();

            stdout().flush().unwrap();
            
            let to_sleep = Instant::now()
                .saturating_duration_since(start)
                .as_nanos()
                .saturating_sub((1.0 / (self.framerate as u32 * 1000000) as f32) as u128) as u64;
            thread::sleep(Duration::from_nanos(to_sleep));
        }
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
