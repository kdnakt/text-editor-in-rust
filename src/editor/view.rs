use std::io::Error;

use super::terminal::{Size, Terminal};

mod buffer;
use buffer::Buffer;

#[derive(Default)]
pub struct View {
    buffer: Buffer,
}

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

impl View {
    pub fn render(&self) -> Result<(), Error> {
        let Size { height, .. } = Terminal::size()?;
        for current_row in 0..height {
            Terminal::clear_line()?;
            if let Some(line) = self.buffer.lines.get(current_row) {
                Terminal::print(line)?;
                Terminal::print("\r\n")?;
                continue;
            }
            #[allow(clippy::integer_division)]
            if current_row == height / 3 {
                Self::draw_welcome_message()?;
            } else {
                Terminal::print("~")?;
            }
            if current_row.saturating_add(1) < height {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }

    fn draw_welcome_message() -> Result<(), Error> {
        let mut welcome_message = format!("{NAME} editor -- version {VERSION}");
        let width = Terminal::size()?.width;
        let len = welcome_message.len();
        #[allow(clippy::integer_division)]
        let padding = (width.saturating_sub(len)) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        welcome_message = format!("~{spaces}{welcome_message}");
        welcome_message.truncate(width);
        Terminal::print(welcome_message)?;
        Ok(())
    }

    pub fn load(&mut self, file_name: &str) {
        if let Ok(buffers) = Buffer::load(file_name) {
            self.buffer = buffers;
        }
    }
}
