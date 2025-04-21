use std::{
    fmt::Display,
    io::{stdout, Error, Write},
};

use crossterm::{
    cursor::MoveTo,
    queue,
    terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType},
    Command,
};

#[derive(Default, Clone, Copy)]
pub struct Size {
    pub height: usize,
    pub width: usize,
}

#[derive(Clone, Copy, Default)]
pub struct Position {
    pub col: usize,
    pub row: usize,
}

/// Represents the Terminal.
pub struct Terminal {}

impl Terminal {
    pub fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_caret_to(Position::default())?;
        Self::execute()?;
        Ok(())
    }

    pub fn show_caret() -> Result<(), Error> {
        Self::queue_command(crossterm::cursor::Show)?;
        Ok(())
    }

    pub fn hide_caret() -> Result<(), Error> {
        Self::queue_command(crossterm::cursor::Hide)?;
        Ok(())
    }

    pub fn print<T: Display>(string: T) -> Result<(), Error> {
        Self::queue_command(crossterm::style::Print(string))?;
        Ok(())
    }

    pub fn print_row(row: usize, line_text: &str) -> Result<(), Error> {
        Self::move_caret_to(Position { col: 0, row })?;
        Self::clear_line()?;
        Self::print(line_text)?;
        Ok(())
    }

    pub fn move_caret_to(position: Position) -> Result<(), Error> {
        let command = MoveTo(position.col as u16, position.row as u16);
        Self::queue_command(command)?;
        Ok(())
    }

    pub fn clear_screen() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::All))?;
        Ok(())
    }

    pub fn clear_line() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::CurrentLine))?;
        Ok(())
    }

    pub fn terminate() -> Result<(), Error> {
        Self::execute()?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn queue_command<T: Command>(command: T) -> Result<(), Error> {
        queue!(stdout(), command)?;
        Ok(())
    }

    pub fn execute() -> Result<(), Error> {
        stdout().flush()?;
        Ok(())
    }

    pub fn size() -> Result<Size, Error> {
        let (width_u16, height_u16) = size()?;
        #[allow(clippy::as_conversions)]
        let height = height_u16 as usize;
        #[allow(clippy::as_conversions)]
        let width = width_u16 as usize;
        Ok(Size { height, width })
    }
}
