use crate::internal_prelude::*;

/// This is compared (and hashed) by address range, not just by value.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SourceFile<'a> {
    ass: Ass<'a>,
}

impl<'a> SourceFile<'a> {
    pub const fn new(raw: &'a str) -> Self {
        Self { ass: Ass { raw } }
    }

    pub const fn raw(self) -> &'a str {
        self.ass.raw
    }

    pub fn substr_range(self, substr: SourceFragment) -> Option<Range<TextPosition>> {
        self.raw()
            .substr_range(substr.raw())
            .map(TextPosition::convert_range)
    }
}
impl Debug for SourceFile<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self.ass, f)
    }
}
impl Display for SourceFile<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.ass, f)
    }
}

/// This is compared (and hashed) by address range, not just by value.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SourceFragment<'a> {
    ass: Ass<'a>,
}

impl<'a> SourceFragment<'a> {
    pub const fn new(raw: &'a str) -> Self {
        Self { ass: Ass { raw } }
    }

    pub const fn raw(self) -> &'a str {
        self.ass.raw
    }
}
impl Debug for SourceFragment<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self.ass, f)
    }
}
impl Display for SourceFragment<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.ass, f)
    }
}

#[derive(Clone, Copy)]
pub struct CompareByValue<T> {
    pub raw: T,
}

pub(crate) type CbvFile<'a> = CompareByValue<SourceFile<'a>>;
pub(crate) type CbvFragment<'a> = CompareByValue<SourceFragment<'a>>;

impl Debug for CbvFile<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(self.raw.raw(), f)
    }
}
impl Display for CbvFile<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(self.raw.raw(), f)
    }
}
impl PartialEq for CbvFile<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.raw.raw() == other.raw.raw()
    }
}
impl Eq for CbvFile<'_> {}
impl PartialOrd for CbvFile<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.raw.raw().partial_cmp(other.raw.raw())
    }
}
impl Hash for CbvFile<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.raw.raw().hash(state)
    }
}

impl Debug for CbvFragment<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(self.raw.raw(), f)
    }
}
impl Display for CbvFragment<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(self.raw.raw(), f)
    }
}
impl PartialEq for CbvFragment<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.raw.raw() == other.raw.raw()
    }
}
impl Eq for CbvFragment<'_> {}
impl PartialOrd for CbvFragment<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.raw.raw().partial_cmp(other.raw.raw())
    }
}
impl Hash for CbvFragment<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.raw.raw().hash(state)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TextPosition {
    pub byte_index: usize,
}

impl TextPosition {
    const fn convert_range(r: Range<usize>) -> Range<Self> {
        Range {
            start: TextPosition {
                byte_index: r.start,
            },
            end: TextPosition { byte_index: r.end },
        }
    }
}

/// Address-sensitive string.
#[derive(Clone, Copy, Default)]
struct Ass<'a> {
    raw: &'a str,
}

impl Debug for Ass<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // (0xbeefcafe..0xfeedcafe) "hello\nworld"
        let range = self.raw.as_bytes().as_ptr_range();
        write!(f, "({:p}..{:p}) {:?}", range.start, range.end, self.raw)
    }
}

impl Display for Ass<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(self.raw, f)
    }
}

impl PartialEq for Ass<'_> {
    fn eq(&self, other: &Self) -> bool {
        (self.raw.as_ptr(), self.raw.len()) == (other.raw.as_ptr(), other.raw.len())
    }
}
impl Eq for Ass<'_> {}

impl PartialOrd for Ass<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Ass<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.raw.as_ptr(), self.raw.len()).cmp(&(other.raw.as_ptr(), other.raw.len()))
    }
}

impl Hash for Ass<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.raw.as_ptr(), self.raw.len()).hash(state)
    }
}
