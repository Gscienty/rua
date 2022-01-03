use super::super::{AstLocal, ExprFunction};

#[derive(Clone)]
pub struct StatLocalFunction {
    name: Box<AstLocal>,
    function: Box<ExprFunction>,
}
