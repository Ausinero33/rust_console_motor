use std::process::Command;

pub struct Window {
    pub width: u8,
    pub height: u8,
    frame: Vec<u8>,
}

impl Window {
    pub fn new() -> Window {
        let size = Window::get_term_size();

        Window {
            width: size.0,
            height: size.1,
            frame: vec![0x24; size.0 as usize * size.1 as usize],
        }
    }

    pub fn draw(&self) {
        Window::clear();
        println!("{}", String::from_utf8_lossy(&self.frame));
    }

    fn clear() {
        // print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        // print!("{}[2J", 27 as char);
        Command::new("clear");
    }

    fn get_term_size() -> (u8, u8) {
        let lines = Command::new("tput")
            .arg("lines")
            .output()
            .expect("Failed to execute command");
        let cols = Command::new("tput")
            .arg("cols")
            .output()
            .expect("Failed to execute command");

        (lines.stdout[0], cols.stdout[0])
    }

}
