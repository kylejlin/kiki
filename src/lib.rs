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
        .map_err(|e| KikiErr::parse_err(src, e))?;
    let ast: data::ast::File = cst.into();
    ast_to_rust(&ast)
}
