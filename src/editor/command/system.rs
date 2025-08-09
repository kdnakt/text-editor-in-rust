use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::prelude::*;

#[derive(Clone, Copy)]
pub enum System {
    Save,
    Search,
    Resize(Size),
    Quit,
    Dismiss,
}

impl TryFrom<KeyEvent> for System {
    type Error = String;

    fn try_from(event: KeyEvent) -> Result<Self, Self::Error> {
        let KeyEvent {
            code, modifiers, ..
        } = event;
        if modifiers == KeyModifiers::CONTROL {
            match code {
                KeyCode::Char('s') => Ok(Self::Save),
                KeyCode::Char('q') => Ok(Self::Quit),
                KeyCode::Char('f') => Ok(Self::Search),
                _ => Err(format!("Unsupported CONTROL+{code:?} combination")),
            }
        } else if modifiers == KeyModifiers::NONE && matches!(code, KeyCode::Esc) {
            Ok(Self::Dismiss)
        } else {
            Err(format!(
                "Unsupported key code {code:?} or modifier {modifiers:?}"
            ))
        }
    }
}
