#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub parser);

pub mod data;
mod pipeline;

#[cfg(test)]
mod tests;

pub use data::*;

use pipeline::prelude::*;

pub fn generate(src: &str) -> Result<RustSrc, KikiErr> {
    let cst = pipeline::parser::FileParser::new()
        .parse(src)
        .map_err(|e| lalr_parse_err_to_kiki_err(src, e))?;
    let ast: data::ast::File = cst.into();
    let validated = pipeline::validate_ast::validate_ast(ast)?;
    let machine = validated_ast_to_machine(&validated);
    let table = machine_to_table(&machine, &validated)?;
    Ok(table_to_rust(&table, &validated))
}
