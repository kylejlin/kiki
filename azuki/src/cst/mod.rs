use crate::internal_prelude::*;

mod kiki_bridge;
mod massage;
mod raw_parser;

#[derive(Debug)]
pub struct File<'a> {
    // pub items: Vec<FileItem<'a>>,
    _todo_delete_me: PhantomData<&'a ()>,
}

// TODO
