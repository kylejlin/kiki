use crate::internal_prelude::*;

pub struct PreflightCertificate<'a> {
    pub(crate) source: &'a str,
    pub(crate) file: &'a cst::File<'a>,
    pub(crate) start_symbol: token::UpperName<'a>,
    pub(crate) terminal: &'a cst::TerminalEnum<'a>,
    pub(crate) bridge_path: &'a cst::Path<'a>,
    // pub(crate) non_terminal_defs: HashMap<SigilFreeStr<'a>, &'a m::FileItem>,
    // pub(crate) terminal_bridge_resolutions: HashMap<SigilFreeStr<'a>, SigilFreeStr<'a>>,
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
///    For example, `enum Foo { Bar { a: $Lorem, b: Ipsum } Baz { x: $Lorem, _: Ipsum } }` is illegal
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
