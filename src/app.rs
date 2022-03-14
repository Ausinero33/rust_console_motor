use std::{io::{stdout, Write}, time::{Instant, Duration}, thread};

use crossterm::{execute, terminal::{size, self, enable_raw_mode, LeaveAlternateScreen, DisableLineWrap, disable_raw_mode, SetTitle}, cursor, event::{read, poll, Event, KeyCode, KeyEvent}, style::Print, queue};

use crate::{core::{object::Object, render::Renderer}, util::vec::Vector3f};

pub struct App {
    framerate: u16,
    renderer: Renderer,
    size: (u16, u16),
    elapsed_time: f64,
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
            elapsed_time: 0.,
        }
    }

    pub fn set_framerate(mut self, framerate: u16) -> App {
        self.framerate = framerate;
        self
    }

    pub fn run(mut self) {
        let mut dt: Duration = Duration::new(0, 0);
        loop {
            let start = Instant::now();
           

            if poll(Duration::from_nanos(5)).unwrap() {
                match read().unwrap() {
                    Event::Key(KeyEvent {code: KeyCode::Char('c'), ..}) => break,
                    Event::Key(KeyEvent {code: KeyCode::Char('w'), ..}) => self.renderer.eye.z += 1. * dt.as_micros() as f32 / 1000000.,
                    Event::Key(KeyEvent {code: KeyCode::Char('s'), ..}) => self.renderer.eye.z -= 1. * dt.as_micros() as f32 / 1000000.,
                    Event::Key(KeyEvent {code: KeyCode::Char('d'), ..}) => self.renderer.eye.x += 1. * dt.as_micros() as f32 / 1000000.,
                    Event::Key(KeyEvent {code: KeyCode::Char('a'), ..}) => self.renderer.eye.x -= 1. * dt.as_micros() as f32 / 1000000.,
                    Event::Key(KeyEvent {code: KeyCode::Char('z'), ..}) => self.renderer.eye.y += 1. * dt.as_micros() as f32 / 1000000.,
                    Event::Key(KeyEvent {code: KeyCode::Char('x'), ..}) => self.renderer.eye.y -= 1. * dt.as_micros() as f32 / 1000000.,
                    _ => {},
                }
            }
            
            queue!(
                stdout(),
                cursor::MoveTo(0, 0),
            ).unwrap();

            // self.update_light(self.elapsed_time / 2.);
            self.renderer.render(&self.size);

            stdout().flush().unwrap();
            
            dt = Instant::now().duration_since(start);
           
            self.elapsed_time += dt.as_secs_f64();

            let str = format!(
                "{:.2}", 1. / (dt.as_micros() as f32 / 1000000.)
            );

            queue!(
                stdout(),
                SetTitle(str)
            ).unwrap();
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

    fn update_light(&mut self, t: f64) {
        self.renderer.light = Vector3f {
            x: self.renderer.initial_light.x + libm::sin(t) as f32 * 5.,
            y: self.renderer.initial_light.y,
            z: self.renderer.initial_light.z + libm::cos(t) as f32 * 5.,
        };
    }
}
