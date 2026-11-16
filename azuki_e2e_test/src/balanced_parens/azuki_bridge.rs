use crate::parser::non_terminals::Expr;

pub type Unit = ();
pub type BoxedExpr = Box<Expr>;

pub fn boxed(inner: Expr) -> BoxedExpr {
    Box::new(inner)
}
