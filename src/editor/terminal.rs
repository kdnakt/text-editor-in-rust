use crossterm::{
    execute,
    terminal::{Clear, ClearType},
};

pub struct Terminal {}

impl Terminal {
    pub fn clear_screen() -> Result<(), std::io::Error> {
        let mut stdout = std::io::stdout();
        execute!(stdout, Clear(ClearType::All))?;
        Ok(())
    }
}
