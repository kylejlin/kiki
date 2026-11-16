// This module contains functions for the _user's_ convenience,
// **not** for the library implementor's convenience.

use crate::internal_prelude::*;

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

pub fn azuki_to_rust(s: &str) -> Result<RustCode, ErrorMessage> {
    todo!()
}
