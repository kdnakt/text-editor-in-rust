use crossterm::event::KeyCode::Char;
use crossterm::{
    event::{read, Event::Key},
    terminal::{disable_raw_mode, enable_raw_mode},
};
pub struct Editor {}

impl Editor {
    pub fn default() -> Self {
        Editor {}
    }

    pub fn run(&self) {
        enable_raw_mode().unwrap();
        loop {
            match read() {
                Ok(Key(event)) => {
                    println!("{:?}\r", event);
                    match event.code {
                        Char(c) => {
                            if c == 'q' {
                                break;
                            }
                        }
                        _ => {}
                    }
                }
                Err(e) => println!("Error: {}", e),
                _ => {}
            }
        }
        disable_raw_mode().unwrap();
    }
}
