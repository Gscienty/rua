use super::super::{AstExpr, ExprFunction};

pub struct StatFunction {
    name: Box<AstExpr>,
    function: Box<ExprFunction>,
}
