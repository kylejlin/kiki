#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub parser);

mod data;
mod logic;

#[cfg(test)]
mod tests;

pub use data::*;

use logic::prelude::*;

pub fn generate(src: &str) -> Result<RustSrc, KikiErr> {
    let cst = logic::parser::FileParser::new()
        .parse(src)
        .map_err(|e| lalr_parse_err_to_kiki_err(src, e))?;
    let ast: data::ast::File = cst.into();
    let validated = logic::validate_ast::validate_ast(ast)?;
    let machine = validated_ast_to_machine(&validated)?;
    let table = machine_to_table(&machine)?;
    Ok(table_to_rust(&table, validated))
}
