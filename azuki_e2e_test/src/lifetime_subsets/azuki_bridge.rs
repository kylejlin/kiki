pub type Text<'src> = &'src str;
pub type Owned = String;

// Deliberately differs from the global lifetime order in the grammar.
#[derive(Debug)]
pub struct BorrowedPair<'right, 'left> {
    pub left: &'left str,
    pub right: &'right str,
}

pub fn own(pair: BorrowedPair<'_, '_>) -> Owned {
    format!("{} {}", pair.left, pair.right)
}
