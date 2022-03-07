use std::{io::{stdout, Write}, time::{Instant, Duration}, thread};

use crossterm::{execute, terminal::{size, self}, cursor, queue, style};

use crate::core::{object::Object, render::Renderer};

pub struct App {
    framerate: u16,
    frame_buffer: Vec<u8>,
    renderer: Renderer,
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
            frame_buffer: vec![0x00; (size.0 * size.1) as usize],
            renderer: Renderer::new(),
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
        self.renderer.add_object(obj);
    }


}
