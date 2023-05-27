use crate::data::{validated_file::*, KikiErr, RustSrc};
use crate::logic::prelude::*;

pub fn ast_to_rust(file: ValidatedFile) -> Result<RustSrc, KikiErr> {
    let machine = ast_to_machine(&file)?;
    let table = machine_to_table(&machine)?;
    table_to_rust(&table, file)
}
