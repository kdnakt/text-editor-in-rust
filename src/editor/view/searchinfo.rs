use crate::editor::{line::Line, position::Position};

use super::location::Location;

pub struct SearchInfo {
    pub prev_location: Location,
    pub prev_scroll_offset: Position,
    pub query: Option<Line>,
}
