use crossterm::style::Color;

pub struct Attribute {
    pub foreground: Option<Color>,
    pub background: Option<Color>,
}
