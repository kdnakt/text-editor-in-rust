use crate::editor::terminal::Position;

#[derive(Clone, Copy, Default)]
pub struct Location {
    pub x: usize,
    pub y: usize,
}

impl From<Location> for Position {
    fn from(location: Location) -> Self {
        Self {
            col: location.x,
            row: location.y,
        }
    }
}

impl Location {
    pub const fn subtract(&self, other: &Self) -> Self {
        Self {
            x: self.x.saturating_sub(other.x),
            y: self.y.saturating_sub(other.y),
        }
    }
}
