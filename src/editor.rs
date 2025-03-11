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
                    println!("{event:?}\r");
                    if let Char(c) = event.code {
                        if c == 'q' {
                            break;
                        }
                    }
                }
                Err(e) => println!("Error: {e}"),
                _ => {}
            }
        }
        disable_raw_mode().unwrap();
    }
}
