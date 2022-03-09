use std::{io::{stdout, Write}, time::{Instant, Duration}, thread};

use crossterm::{execute, terminal::{size, self, enable_raw_mode, LeaveAlternateScreen, DisableLineWrap, disable_raw_mode, SetTitle}, cursor, event::{read, poll, Event}, style::Print};

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
            let start = Instant::now();
            
            if poll(Duration::from_nanos(5)).unwrap() {
                match read().unwrap() {
                    Event::Key(_event) => break,
                    _ => {},
                }
            }
            
            execute!(
                stdout(),
                cursor::MoveTo(0, 0),
            ).unwrap();

            self.renderer.render(&self.size);

            stdout().flush().unwrap();
            
            let dt = Instant::now().duration_since(start);
            
            // Arreglar esto

            let to_sleep = dt
                .as_micros()
                .saturating_sub((1. / (self.framerate as f32 * 1000000.)) as u128);

            let str = format!(
                "{:.2}", 1. / (dt.as_micros() as f32 / 1000000.)
            );

            execute!(
                stdout(),
                SetTitle(str)
            ).unwrap();

            thread::sleep(Duration::from_micros(to_sleep as u64));
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
