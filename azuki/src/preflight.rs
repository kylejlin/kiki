use crate::internal_prelude::*;

pub struct PreflightCertificate<'a> {
    pub(crate) source: &'a str,
    pub(crate) file: &'a cst::File<'a>,
    pub(crate) start_symbol: token::UpperName<'a>,
    pub(crate) terminal_enum: &'a cst::TerminalEnum<'a>,
    pub(crate) bridge_path: &'a cst::Path<'a>,
    pub(crate) terminal_defs: TerminalIdMap<&'a cst::TerminalEnumVariant<'a>>,
    pub(crate) non_terminal_defs: OriginalNonTerminalIdMap<&'a cst::FileItem<'a>>,

    /// The first rule is always the synthetic rule (i.e., `{synthetic_start} <- {original_start}`).
    pub(crate) rules: Vec<Rule>,
}

#[derive(Debug, Clone)]
pub enum PreflightCheckError {
    Todo,
}

/// Precondition: `file` must be the parse result of `source`.
///
/// Postcondition: Returns `Ok` if and only if all of the following hold:
/// 1. There is exactly one `start` item.
/// 2. There is exactly one `terminal` item.
/// 3. There is exactly one `bridge` item.
/// 4. No two items have the same name.
/// 5. For any given `enum` or `terminal` item, no two variants have the same name.
/// 6. For any given `enum` or `manual` item, no two variants have the same symbol sequence.
///    For example, `enum Foo { Bar { a: $Lorem b: Ipsum } Baz { x: $Lorem _: Ipsum } }` is illegal
///    (since both `Bar` and `Baz` have the sequence `[$Lorem, Ipsum]`).
/// 7. For any given named field set, no two fields have the same name (obviously, underscores are exempt).
/// 8. Every terminal grammar symbol occurrence is defined in the `terminal` item.
/// 9. Every non-terminal grammar symbol occurrence (including the `start` symbol) is defined by a non-terminal item (i.e., `struct`, `enum`, or `manual`).
/// 10. For any given lifetime list, there are no duplicates.
/// 11. Any given lifetime list is a non-strict subset of the `terminal` item's global lifetime set.
pub fn preflight_check<'a>(
    source: &'a str,
    file: &'a cst::File<'a>,
) -> Result<PreflightCertificate<'a>, PreflightCheckError> {
    todo!()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SymbolId {
    raw: NonZeroUsize,
}

impl SymbolId {
    const HIGH_BIT_MASK: usize = 1 << (usize::BITS - 1);

    pub const fn as_terminal(self) -> Result<TerminalId, NonTerminalId> {
        let Self { raw } = self;

        if (raw.get() & Self::HIGH_BIT_MASK) == 0 {
            Ok(TerminalId { raw })
        } else {
            Err(NonTerminalId { raw })
        }
    }

    pub const fn as_non_terminal(self) -> Result<NonTerminalId, TerminalId> {
        let Self { raw } = self;

        if (raw.get() & Self::HIGH_BIT_MASK) == 0 {
            Err(TerminalId { raw })
        } else {
            Ok(NonTerminalId { raw })
        }
    }

    pub const fn from_terminal(id: TerminalId) -> Self {
        Self { raw: id.raw }
    }

    pub const fn from_non_terminal(id: NonTerminalId) -> Self {
        Self { raw: id.raw }
    }
}
impl From<TerminalId> for SymbolId {
    fn from(id: TerminalId) -> Self {
        Self::from_terminal(id)
    }
}
impl From<NonTerminalId> for SymbolId {
    fn from(id: NonTerminalId) -> Self {
        Self::from_non_terminal(id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TerminalId {
    /// INVARIANT: The most significant bit is zero.
    raw: NonZeroUsize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NonTerminalId {
    /// INVARIANT: The most significant bit is one.
    raw: NonZeroUsize,
}

pub struct TerminalIdMap<T> {
    raw: Vec<T>,
}

impl<T> Index<NonTerminalId> for TerminalIdMap<T> {
    type Output = T;

    fn index(&self, index: NonTerminalId) -> &Self::Output {
        let raw_index = index.raw.get();
        let index = raw_index - 1;
        &self.raw[index]
    }
}

pub struct OriginalNonTerminalIdMap<T> {
    raw: Vec<T>,
}

impl<T> Index<NonTerminalId> for OriginalNonTerminalIdMap<T> {
    type Output = T;

    fn index(&self, index: NonTerminalId) -> &Self::Output {
        let raw_index = index.raw.get() & !SymbolId::HIGH_BIT_MASK;

        // _Original_ non-terminals have a non-zero index.
        // The zeroth index is reserved for the synthetic start symbol.
        let index = raw_index.checked_sub(1).unwrap();
        &self.raw[index]
    }
}

pub struct Rule {
    pub lhs: NonTerminalId,
    pub rhs: Vec<SymbolId>,
}
