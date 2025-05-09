use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

use super::terminal::Size;

pub enum Direction {
    PageUp,
    PageDown,
    Home,
    End,
    Up,
    Down,
    Left,
    Right,
}

pub enum EditorCommand {
    Move(Direction),
    Resize(Size),
    Quit,
    Insert(char),
    Backspace,
    Delete,
    Enter,
    Save,
}

#[allow(clippy::as_conversions)]
impl TryFrom<Event> for EditorCommand {
    type Error = String;

    fn try_from(event: Event) -> Result<Self, Self::Error> {
        match event {
            Event::Key(KeyEvent {
                code, modifiers, ..
            }) => match (code, modifiers) {
                (KeyCode::Char('q'), KeyModifiers::CONTROL) => Ok(Self::Quit),
                (KeyCode::Char('s'), KeyModifiers::CONTROL) => Ok(Self::Save),
                (KeyCode::Char(character), KeyModifiers::NONE | KeyModifiers::SHIFT) => {
                    Ok(Self::Insert(character))
                }
                (KeyCode::Tab, _) => Ok(Self::Insert('\t')),
                (KeyCode::Enter, _) => Ok(Self::Enter),
                (KeyCode::Up, _) => Ok(Self::Move(Direction::Up)),
                (KeyCode::Down, _) => Ok(Self::Move(Direction::Down)),
                (KeyCode::Left, _) => Ok(Self::Move(Direction::Left)),
                (KeyCode::Right, _) => Ok(Self::Move(Direction::Right)),
                (KeyCode::PageUp, _) => Ok(Self::Move(Direction::PageUp)),
                (KeyCode::PageDown, _) => Ok(Self::Move(Direction::PageDown)),
                (KeyCode::Home, _) => Ok(Self::Move(Direction::Home)),
                (KeyCode::End, _) => Ok(Self::Move(Direction::End)),
                (KeyCode::Backspace, _) => Ok(Self::Backspace),
                (KeyCode::Delete, _) => Ok(Self::Delete),
                _ => Err(format!("Key code not supported: {code:?}")),
            },
            Event::Resize(width_u16, height_u16) => Ok(Self::Resize(Size {
                height: height_u16 as usize,
                width: width_u16 as usize,
            })),
            _ => Err(format!("Unsupported event: {event:?}")),
        }
    }
}
