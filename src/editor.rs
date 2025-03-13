use crossterm::event::KeyCode::Char;
use crossterm::{
    event::{read, Event::Key},
    terminal::{disable_raw_mode, enable_raw_mode},
};
pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn default() -> Self {
        Self { should_quit: false }
    }

    pub fn run(&self) {
        if let Err(e) = self.repl() {
            panic!("{e:#?}");
        }
        print!("Goodbye.\r\n");
    }

    fn repl(&self) -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        loop {
            if let Key(event) = read()? {
                println!("{event:?}\r");
                if let Char(c) = event.code {
                    if c == 'q' {
                        break;
                    }
                }
            }
        }
        disable_raw_mode()?;
        Ok(())
    }
}
