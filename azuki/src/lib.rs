pub mod bit_set;
pub mod codegen;
pub mod convenience;
pub mod cst;
pub mod lexer;
pub mod ntt;
pub mod preflight;
pub mod source_text;
pub mod token;

mod internal_prelude;

mod machine;
mod oset;
mod table;
mod takeoff;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct RustCode {
    pub raw: String,
}
