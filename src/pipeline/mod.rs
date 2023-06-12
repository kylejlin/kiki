pub mod cst_to_ast;
pub mod machine_to_table;
pub mod normalize_machine;
pub mod sort_and_get_index_updater;
pub mod table_to_rust;
pub mod validate_ast;
pub mod validated_ast_to_machine;

pub use crate::parser;

pub mod prelude {
    pub use super::cst_to_ast::*;
    pub use super::machine_to_table::*;
    pub use super::table_to_rust::*;
    pub use super::validated_ast_to_machine::*;
}
