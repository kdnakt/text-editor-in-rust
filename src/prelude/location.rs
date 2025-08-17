use super::{GraphemeIdx, LineIdx};

#[derive(Clone, Copy, Default)]
pub struct Location {
    pub grapheme_index: GraphemeIdx,
    pub line_index: LineIdx,
}
