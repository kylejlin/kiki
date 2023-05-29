use crate::data::{ast::*, validated_file::*, KikiErr};

pub fn ast_to_validated_file(file: File) -> Result<ValidatedFile, KikiErr> {
    let start = get_start(&file)?;
    todo!()
}

fn get_start(file: &File) -> Result<String, KikiErr> {
    let starts: Vec<&Ident> = file
        .items
        .iter()
        .filter_map(|item| match item {
            Item::Start(start) => Some(start),
            _ => None,
        })
        .collect();
    if starts.len() == 0 {
        Err(KikiErr::NoStartSymbol)
    } else if starts.len() > 1 {
        let positions = starts.iter().map(|start| start.position).collect();
        Err(KikiErr::MultipleStartSymbols(positions))
    } else {
        Ok(starts[0].name.clone())
    }
}
