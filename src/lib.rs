#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub parser);

mod cst;

#[cfg(test)]
mod tests;
