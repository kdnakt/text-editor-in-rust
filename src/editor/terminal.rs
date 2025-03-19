use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};

pub struct Terminal {}

impl Terminal {
    pub fn initialize() -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::draw_rows()?;
        Self::move_cursor_to(0, 0)?;
        Ok(())
    }

    pub fn move_cursor_to(x: u16, y: u16) -> Result<(), std::io::Error> {
        let command = crossterm::cursor::MoveTo(x, y);
        execute!(std::io::stdout(), command)?;
        Ok(())
    }

    pub fn clear_screen() -> Result<(), std::io::Error> {
        let mut stdout = std::io::stdout();
        execute!(stdout, Clear(ClearType::All))?;
        Ok(())
    }

    pub fn draw_rows() -> Result<(), std::io::Error> {
        for _ in 0..crossterm::terminal::size().unwrap().1 {
            print!("~\r\n");
        }
        let command = crossterm::cursor::MoveTo(0, 0);
        crossterm::execute!(std::io::stdout(), command)?;
        Ok(())
    }

    pub fn terminate() -> Result<(), std::io::Error> {
        disable_raw_mode()?;
        Ok(())
    }
}
