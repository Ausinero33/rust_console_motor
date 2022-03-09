use std::{io::{stdout, Write}, time::{Instant, Duration}, thread};

use crossterm::{execute, terminal::{size, self, enable_raw_mode, LeaveAlternateScreen, DisableLineWrap, disable_raw_mode}, cursor, event::{read, poll, Event}};

use crate::core::{object::Object, render::Renderer};

pub struct App {
    framerate: u16,
    renderer: Renderer,
    size: (u16, u16)
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

        enable_raw_mode().unwrap();

        App {
            framerate: 24,
            renderer: Renderer::new(),
            size,
        }
    }

    pub fn set_framerate(mut self, framerate: u16) -> App {
        self.framerate = framerate;
        self
    }

    pub fn run(self) {
        loop {
            if poll(Duration::from_nanos(5)).unwrap() {
                match read().unwrap() {
                    Event::Key(_event) => break,
                    _ => {},
                }
            }


            let start = Instant::now();
            
            execute!(
                stdout(),
                cursor::MoveTo(0, 0),
                // style::Print(String::from_utf8_lossy(self.frame_buffer.as_slice())),
            ).unwrap();

            self.renderer.render(&self.size);
            
            stdout().flush().unwrap();
            
            let to_sleep = Instant::now()
                .duration_since(start)
                .as_nanos()
                .saturating_sub((1.0 / (self.framerate as u32 * 1000000) as f32) as u128) as u64;
            thread::sleep(Duration::from_nanos(to_sleep));
        }

        execute!(stdout(), 
            LeaveAlternateScreen, 
            DisableLineWrap, 
            cursor::Show
        ).unwrap();

        disable_raw_mode().unwrap();
    }

    pub fn add_object(&mut self, obj: Box<dyn Object>) {
        self.renderer.add_object(obj);
    }


}
