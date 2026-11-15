use super::*;

use raw_parser as r;

pub(super) trait Massage<'a> {
    type Out;

    fn massage(&self, file_source: FileSource<'a>) -> Self::Out;
}

impl<'a> Massage<'a> for r::File {
    type Out = File<'a>;

    fn massage(&self, file_source: FileSource<'a>) -> Self::Out {
        todo!()
    }
}
