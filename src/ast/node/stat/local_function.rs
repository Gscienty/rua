use super::super::{AstLocal, ExprFunction};

pub struct StatLocalFunction {
    name: Box<AstLocal>,
    function: Box<ExprFunction>,
}
