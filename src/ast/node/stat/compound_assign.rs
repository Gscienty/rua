use super::super::{AstExpr, BinaryOperator};

#[derive(Clone)]
pub struct StatCompoundAssign {
    operator: BinaryOperator,
    var: Box<AstExpr>,
    value: Box<AstExpr>,
}
