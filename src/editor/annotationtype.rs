#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AnnotationType {
    Match,
    SelectedMatch,
    Number,
    Keyword,
    Type,
    KnownValue,
    Char,
}
