use crate::editor::line::Line;
use crate::prelude::*;

use super::location::Location;

pub struct SearchInfo {
    pub prev_location: Location,
    pub prev_scroll_offset: Position,
    pub query: Option<Line>,
}
