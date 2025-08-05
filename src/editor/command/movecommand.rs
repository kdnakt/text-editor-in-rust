use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

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

impl TryFrom<KeyEvent> for Move {
    type Error = String;

    fn try_from(event: KeyEvent) -> Result<Self, Self::Error> {
        let KeyEvent {
            code, modifiers, ..
        } = event;
        if modifiers == KeyModifiers::NONE {
            match code {
                KeyCode::Up => Ok(Self::Up),
                KeyCode::Down => Ok(Self::Down),
                KeyCode::Left => Ok(Self::Left),
                KeyCode::Right => Ok(Self::Right),
                KeyCode::PageUp => Ok(Self::PageUp),
                KeyCode::PageDown => Ok(Self::PageDown),
                KeyCode::Home => Ok(Self::Home),
                KeyCode::End => Ok(Self::End),
                _ => Err(format!("Unsupported key code for Move: {:?}", event.code)),
            }
        } else {
            Err(format!(
                "Unsupported key code {code:?} or modifier {modifiers:?}"
            ))
        }
    }
}
