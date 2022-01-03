use super::super::{AstExpr, AstType};

#[derive(Clone)]
pub struct ExprTypeAssertion {
    expr: Box<AstExpr>,
    annotation: Box<AstType>,
}
