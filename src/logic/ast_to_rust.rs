use crate::data::{ast::*, KikiErr, RustSrc};
use crate::logic::prelude::*;

pub fn ast_to_rust(file: &File) -> Result<RustSrc, KikiErr> {
    let machine = ast_to_machine(file)?;
    let table = machine_to_table(&machine)?;
    table_to_rust(&table, todo!())
}
