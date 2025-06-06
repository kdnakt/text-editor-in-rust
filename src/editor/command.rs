use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

use super::terminal::Size;

#[derive(Clone, Copy)]
pub enum Move {
    PageUp,
    PageDown,
    Home,
    End,
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy)]
pub enum EditorCommand {
    Move(Move),
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
                (KeyCode::Up, _) => Ok(Self::Move(Move::Up)),
                (KeyCode::Down, _) => Ok(Self::Move(Move::Down)),
                (KeyCode::Left, _) => Ok(Self::Move(Move::Left)),
                (KeyCode::Right, _) => Ok(Self::Move(Move::Right)),
                (KeyCode::PageUp, _) => Ok(Self::Move(Move::PageUp)),
                (KeyCode::PageDown, _) => Ok(Self::Move(Move::PageDown)),
                (KeyCode::Home, _) => Ok(Self::Move(Move::Home)),
                (KeyCode::End, _) => Ok(Self::Move(Move::End)),
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
