use crate::internal_prelude::*;

pub(crate) struct CodeGenerator<'a> {
    pub takeoff: &'a TakeoffCertificate<'a>,
    pub machine: &'a Machine,
    pub table: &'a Table,
}

impl CodeGenerator<'_> {
    pub fn codegen(self) -> RustCode {
        todo!()
    }
}
