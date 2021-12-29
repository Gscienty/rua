use super::super::{AstExpr, BinaryOperator};

pub struct StatCompoundAssign {
    operator: BinaryOperator,
    var: Box<AstExpr>,
    value: Box<AstExpr>,
}
