use crate::data::ast::*;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct ValidatedFile {
    pub raw: File,
    pub start: String,
    pub terminal_enum: String,
    pub used_identifiers: HashSet<String>,
}
