use super::super::{AstExpr, AstNodePayload, LexLocation};

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
    pub fn new(location: LexLocation, operator: UnaryOperator, expr: Box<AstExpr>) -> Box<AstExpr> {
        AstExpr::new(
            location,
            AstNodePayload::ExprUnary(Box::new(ExprUnary { operator, expr })),
        )
    }
}
