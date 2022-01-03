use super::super::{AstExpr, AstNodePayload};

#[derive(Clone)]
pub enum UnaryOperator {
    Not,
    Minus,
    Len,
}

#[derive(Clone)]
pub struct ExprUnary {
    operator: UnaryOperator,
    expr: Box<AstExpr>,
}

impl ExprUnary {
    pub fn new(operator: UnaryOperator, expr: Box<AstExpr>) -> AstNodePayload {
        AstNodePayload::ExprUnary(Box::new(ExprUnary { operator, expr }))
    }
}
