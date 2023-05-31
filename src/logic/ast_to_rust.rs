use crate::data::{validated_file as validated, KikiErr, RustSrc};
use crate::logic::prelude::*;

pub fn ast_to_rust(file: validated::File) -> Result<RustSrc, KikiErr> {
    let machine = ast_to_machine(&file)?;
    let table = machine_to_table(&machine)?;
    Ok(table_to_rust(&table, file))
}
