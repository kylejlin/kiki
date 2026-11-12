use crate::internal_prelude::*;

#[derive(Debug, Clone)]
pub enum PreflightCheckError {
    // TODO
}

pub struct PreflightCertificate<'a> {
    pub(crate) non_terminal_defs: HashMap<SigilFreeStr<'a>, &'a m::FileItem>,
    pub(crate) terminal_defs: HashMap<SigilFreeStr<'a>, &'a m::TerminalEnumVariant>,
}

/// Precondition: `file` must be the parse result of `source`.
///
/// Postcondition: Returns `Ok` if and only if all of the following hold:
/// 1. There is exactly one `start` item.
/// 2. There is exactly one `terminal` item.
/// 2. There is exactly one `bridge` item.
/// 3. No two items have the same name.
/// 4. For any given `enum` or `terminal` item, no two variants have the same name.
/// 5. For any given named field set, no two fields have the same name (obviously, underscores are exempt).
/// 6. Every terminal grammar symbol occurrence (including the `start` item) is defined in the `terminal` item.
/// 7. Every non-terminal grammar symbol occurrence is defined by a non-terminal item (i.e., `struct`, `enum`, or `manual`).
pub fn preflight_check<'a>(
    source: &'a str,
    file: &'a m::File,
) -> Result<PreflightCertificate<'a>, PreflightCheckError> {
    todo!()
}
