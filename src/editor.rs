use crossterm::event::KeyCode::Char;
use crossterm::event::{KeyEvent, KeyModifiers};
use crossterm::execute;
use crossterm::terminal::{Clear, ClearType};
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

    pub fn run(&mut self) {
        Self::initialize().unwrap();
        if let Err(e) = self.repl() {
            panic!("{e:#?}");
        }
        print!("Goodbye.\r\n");
    }

    fn initialize() -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        Self::clear_screen()
    }

    fn clear_screen() -> Result<(), std::io::Error> {
        let mut stdout = std::io::stdout();
        execute!(stdout, Clear(ClearType::All))
    }

    fn repl(&mut self) -> Result<(), std::io::Error> {
        loop {
            if let Key(KeyEvent {
                code,
                modifiers,
                kind,
                state,
            }) = read()?
            {
                println!(
                    "Code: {code:?} Modifiers: {modifiers:?} Kind: {kind:?} State: {state:?} \r"
                );
                match code {
                    Char('q') if modifiers == KeyModifiers::CONTROL => {
                        self.should_quit = true;
                    }
                    _ => (),
                }
                if self.should_quit {
                    break;
                }
            }
        }
        disable_raw_mode()?;
        Ok(())
    }
}
