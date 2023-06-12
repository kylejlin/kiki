#![warn(clippy::all)]

pub mod data;
mod pipeline;

mod parser;

#[cfg(test)]
mod tests;

pub use data::*;

use pipeline::prelude::*;

pub fn generate(src: &str) -> Result<RustSrc, KikiErr> {
    let tokens = tokenize(src)?;
    let cst = parse(tokens).map_err(unexpected_token_or_eof_to_kiki_err)?;
    let ast: data::ast::File = cst.into();
    let validated = validate_ast(ast)?;
    let machine = validated_ast_to_machine(&validated);
    let table = machine_to_table(&machine, &validated)?;
    Ok(table_to_rust(&table, &validated, src))
}

pub fn get_grammar_hash(src: RustSrcRef) -> Option<&str> {
    const HASH_PREFIX: &str = "// @sha256 ";
    for line in src.0.lines() {
        if !line.starts_with("//") {
            return None;
        }

        if line.starts_with(HASH_PREFIX) {
            let hash = line.trim_start_matches(HASH_PREFIX);
            return Some(hash);
        }
    }
    None
}
