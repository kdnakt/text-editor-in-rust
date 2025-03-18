use crossterm::{
    execute,
    terminal::{disable_raw_mode, Clear, ClearType},
};

pub struct Terminal {}

impl Terminal {
    pub fn clear_screen() -> Result<(), std::io::Error> {
        let mut stdout = std::io::stdout();
        execute!(stdout, Clear(ClearType::All))?;
        Ok(())
    }

    pub fn terminate() -> Result<(), std::io::Error> {
        disable_raw_mode()?;
        Ok(())
    }
}
