use super::super::{AstExpr, ExprFunction};

#[derive(Clone)]
pub struct StatFunction {
    name: Box<AstExpr>,
    function: Box<ExprFunction>,
}
