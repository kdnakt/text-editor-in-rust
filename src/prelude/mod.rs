pub type GraphemeIdx = usize;
pub type ByteIdx = usize;
pub type ColIdx = usize;
pub type Row = usize;
pub type Col = usize;

mod location;
pub use location::Location;
mod position;
pub use position::Position;
mod size;
pub use size::Size;
