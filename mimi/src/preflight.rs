use crate::internal_prelude::*;

#[derive(Debug, Clone)]
pub enum PreflightCheckError {
    // TODO
}

pub struct PreflightCertificate<'a> {
    pub(crate) start_symbol: span_of::UpperName,
    pub(crate) terminal: &'a m::TerminalEnum,
    pub(crate) bridge_path: &'a m::Path,
    pub(crate) non_terminal_defs: HashMap<SigilFreeStr<'a>, &'a m::FileItem>,
    pub(crate) terminal_resolutions: HashMap<SigilFreeStr<'a>, SigilFreeStr<'a>>,
}

/// Precondition: `file` must be the parse result of `source`.
///
/// Postcondition: Returns `Ok` if and only if all of the following hold:
/// 1. There is exactly one `start` item.
/// 2. There is exactly one `terminal` item.
/// 2. There is exactly one `bridge` item.
/// 3. No two items have the same name.
/// 4. For any given `enum` or `terminal` item, no two variants have the same name.
/// 5. For any given `enum` or `manual` item, no two variants have the same symbol sequence.
///    For example, `enum Foo { Bar { a: $Lorem, b: Ipsum } Baz { x: $Lorem, _: Ipsum } }` is illegal
///    (since both `Bar` and `Baz` have the sequence `[$Lorem, Ipsum]`).
/// 6. For any given named field set, no two fields have the same name (obviously, underscores are exempt).
/// 7. Every terminal grammar symbol occurrence is defined in the `terminal` item.
/// 8. Every non-terminal grammar symbol occurrence (including the `start` symbol) is defined by a non-terminal item (i.e., `struct`, `enum`, or `manual`).
pub fn preflight_check<'a>(
    source: &'a str,
    file: &'a m::File,
) -> Result<PreflightCertificate<'a>, PreflightCheckError> {
    todo!()
}

pub use refined_str::*;
mod refined_str {
    use super::*;

    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
    pub struct SigilFreeStr<'a> {
        raw: &'a str,
    }
    impl Debug for SigilFreeStr<'_> {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            self.raw.fmt(f)
        }
    }
    impl<'a> SigilFreeStr<'a> {
        pub fn new(raw: &'a str) -> Option<Self> {
            if raw.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
                Some(Self { raw })
            } else {
                None
            }
        }

        pub const fn raw(self) -> &'a str {
            self.raw
        }
    }
}
