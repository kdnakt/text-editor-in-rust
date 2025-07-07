use crate::editor::line::Line;

use super::location::Location;

pub struct SearchInfo {
    pub prev_location: Location,
    pub query: Line,
}
