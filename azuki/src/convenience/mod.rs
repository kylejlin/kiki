// This module contains functions for the _user's_ convenience,
// **not** for the library implementor's convenience.

use crate::internal_prelude::*;

use diagnostic::ErrorFormatter;

mod diagnostic;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ErrorMessage {
    raw: String,
}

impl ErrorMessage {
    pub fn into_string(self) -> String {
        self.raw
    }

    pub fn as_str(&self) -> &str {
        &self.raw
    }
}

impl Deref for ErrorMessage {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.raw
    }
}

impl Debug for ErrorMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(self, f)
    }
}
impl Display for ErrorMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.raw, f)
    }
}

/// Generates Rust from an Azuki source file, returning a human-readable diagnostic on error.
///
/// `file_name` is used only in diagnostics and is printed as supplied.
/// Lines and character columns start at 1.
pub fn azuki_to_rust(source: FileSource<'_>, file_name: &str) -> Result<RustCode, ErrorMessage> {
    let errors = ErrorFormatter { source, file_name };
    let file = cst::parse(source).map_err(|error| match error {
        Ok(error) => errors.parse_error(error),
        Err(error) => errors.lex_error(error),
    })?;
    let preflight =
        preflight_check(source.raw(), &file).map_err(|error| errors.preflight_error(error))?;
    let takeoff = TakeoffCertificate::new(preflight);
    let machine = Machine::new(&takeoff);
    let table = Table::new(&machine, &takeoff).map_err(|error| ErrorMessage {
        raw: format!(
            "{:?}",
            TableConflictDiagnostic {
                error,
                machine: &machine,
                takeoff: &takeoff,
                file_name,
            }
        ),
    })?;
    Ok(CodeGenerator {
        takeoff: &takeoff,
        machine: &machine,
        table: &table,
    }
    .codegen())
}
