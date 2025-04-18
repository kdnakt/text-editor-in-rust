use std::io::Error;

use super::{
    editorcommand::{Direction, EditorCommand},
    terminal::{Position, Size, Terminal},
};

mod buffer;
use buffer::Buffer;
mod location;
use location::Location;

pub struct View {
    buffer: Buffer,
    needs_redraw: bool,
    size: Size,
    location: Location,
    scroll_offset: Location,
}

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

impl View {
    pub fn resize(&mut self, to: Size) {
        self.size = to;
        self.needs_redraw = true;
    }

    pub fn render_line(at: usize, line_text: &str) -> Result<(), Error> {
        Terminal::move_caret_to(Position { row: at, col: 0 })?;
        Terminal::clear_line()?;
        Terminal::print(line_text)?;
        Ok(())
    }

    pub fn render(&mut self) -> Result<(), Error> {
        if !self.needs_redraw {
            return Ok(());
        }
        let Size { height, width } = self.size;
        if height == 0 || width == 0 {
            return Ok(());
        }
        #[allow(clippy::integer_division)]
        let vertical_center = height / 3;

        for current_row in 0..height {
            if let Some(line) = self.buffer.lines.get(current_row) {
                let truncated_line = if line.len() >= width {
                    &line[..width]
                } else {
                    line
                };
                Self::render_line(current_row, truncated_line)?;
            } else if current_row == vertical_center && self.buffer.is_empty() {
                Self::render_line(current_row, &Self::build_welcome_message(width))?;
            } else {
                Self::render_line(current_row, "~")?;
            }
        }
        self.needs_redraw = false;
        Ok(())
    }

    fn build_welcome_message(width: usize) -> String {
        let welcome_message = format!("{NAME} editor -- version {VERSION}");
        let len = welcome_message.len();
        if width <= len {
            return "~".to_string();
        }
        #[allow(clippy::integer_division)]
        let padding = (width.saturating_sub(len).saturating_sub(1)) / 2;
        let mut full_message = format!("~{}{}", " ".repeat(padding), welcome_message);
        full_message.truncate(width);
        full_message
    }

    pub fn load(&mut self, file_name: &str) {
        if let Ok(buffers) = Buffer::load(file_name) {
            self.buffer = buffers;
            self.needs_redraw = true;
        }
    }

    pub fn get_position(&self) -> Position {
        self.location.subtract(&self.scroll_offset).into()
    }

    pub fn handle_command(&mut self, command: EditorCommand) {
        match command {
            EditorCommand::Resize(size) => self.resize(size),
            EditorCommand::Move(direction) => self.move_text_location(&direction),
            EditorCommand::Quit => {}
        }
    }

    fn move_text_location(&mut self, direction: &Direction) {
        let Location { mut x, mut y } = self.location;
        let Size { height, width } = self.size;
        match direction {
            Direction::Up => {
                y = y.saturating_sub(1);
            }
            Direction::Down => {
                y = y.saturating_add(1);
            }
            Direction::Left => {
                x = x.saturating_sub(1);
            }
            Direction::Right => {
                x = x.saturating_add(1);
            }
            Direction::PageUp => {
                y = 0;
            }
            Direction::PageDown => {
                y = height.saturating_sub(1);
            }
            Direction::Home => {
                x = 0;
            }
            Direction::End => {
                x = width.saturating_sub(1);
            }
        }
        self.location = Location { x, y };
    }
}

impl Default for View {
    fn default() -> Self {
        let size = Terminal::size().unwrap_or_default();
        Self {
            buffer: Buffer::default(),
            needs_redraw: true,
            size,
            location: Location::default(),
            scroll_offset: Location::default(),
        }
    }
}
