use super::super::{AstExpr, AstType};

pub struct ExprTypeAssertion {
    expr: Box<AstExpr>,
    annotation: Box<AstType>,
}
