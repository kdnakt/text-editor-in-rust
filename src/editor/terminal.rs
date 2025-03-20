use std::io::Error;

use crossterm::{
    execute, queue,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};

pub struct Terminal {}

impl Terminal {
    pub fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::draw_rows()?;
        Self::move_cursor_to(0, 0)?;
        Ok(())
    }

    pub fn show_cursor() -> Result<(), Error> {
        queue!(std::io::stdout(), crossterm::cursor::Show)?;
        Ok(())
    }

    pub fn hide_cursor() -> Result<(), Error> {
        queue!(std::io::stdout(), crossterm::cursor::Hide)?;
        Ok(())
    }

    pub fn print(string: &str) -> Result<(), Error> {
        queue!(std::io::stdout(), crossterm::style::Print(string))?;
        Ok(())
    }

    pub fn move_cursor_to(x: u16, y: u16) -> Result<(), Error> {
        let command = crossterm::cursor::MoveTo(x, y);
        execute!(std::io::stdout(), command)?;
        Ok(())
    }

    pub fn clear_screen() -> Result<(), Error> {
        let mut stdout = std::io::stdout();
        execute!(stdout, Clear(ClearType::All))?;
        Ok(())
    }

    pub fn draw_rows() -> Result<(), Error> {
        for _ in 0..crossterm::terminal::size().unwrap().1 {
            print!("~\r\n");
        }
        let command = crossterm::cursor::MoveTo(0, 0);
        crossterm::execute!(std::io::stdout(), command)?;
        Ok(())
    }

    pub fn terminate() -> Result<(), Error> {
        disable_raw_mode()?;
        Ok(())
    }
}
