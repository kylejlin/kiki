pub mod cst_to_ast;
pub mod machine_to_table;
pub mod normalize_machine;
pub mod sort_and_get_index_updater;
pub mod table_to_rust;
pub mod tokenize;
pub mod unexpected_token_or_eof_to_kiki_err;
pub mod validate_ast;
pub mod validated_ast_to_machine;

pub(crate) use crate::parser;

pub mod prelude {
    pub use super::cst_to_ast::*;
    pub use super::machine_to_table::*;
    pub use super::parser::parse;
    pub use super::table_to_rust::*;
    pub use super::tokenize::*;
    pub use super::unexpected_token_or_eof_to_kiki_err::*;
    pub use super::validate_ast::*;
    pub use super::validated_ast_to_machine::*;
}
