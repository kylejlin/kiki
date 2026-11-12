#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TextPosition {
    pub byte_index: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Span {
    /// Inclusive
    pub start: TextPosition,

    /// Exclusive
    pub end: TextPosition,
}
