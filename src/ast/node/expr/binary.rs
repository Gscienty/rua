use super::super::{AstExpr, AstNodePayload, LexLocation};

#[derive(Clone)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    Concat,
    NotEqual,
    Equal,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    And,
    Or,
}

#[derive(Clone)]
pub struct ExprBinary {
    operator: BinaryOperator,
    left: Box<AstExpr>,
    right: Box<AstExpr>,
}

impl ExprBinary {
    pub fn new(
        location: LexLocation,
        operator: BinaryOperator,
        left: Box<AstExpr>,
        right: Box<AstExpr>,
    ) -> Box<AstExpr> {
        AstExpr::new(
            location,
            AstNodePayload::ExprBinary(Box::new(ExprBinary {
                operator,
                left,
                right,
            })),
        )
    }
}
