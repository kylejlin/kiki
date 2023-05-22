use crate::data::{ast::*, KikiErr, RustSrc};

impl TryFrom<&File> for RustSrc {
    type Error = KikiErr;

    fn try_from(file: &File) -> Result<Self, Self::Error> {
        todo!()
    }
}
