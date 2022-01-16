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

impl BinaryOperator {
    pub fn left_priority(&self) -> usize {
        match self {
            &BinaryOperator::Or => 1,
            &BinaryOperator::And => 2,
            &BinaryOperator::NotEqual | &BinaryOperator::Equal
            | &BinaryOperator::Less | &BinaryOperator::LessEqual
            | &BinaryOperator::Greater | &BinaryOperator::GreaterEqual => 3,
            &BinaryOperator::Concat => 5,
            &BinaryOperator::Add | &BinaryOperator::Sub => 6,
            &BinaryOperator::Mul | &BinaryOperator::Div | &BinaryOperator::Mod => 7,
            &BinaryOperator::Pow => 10,
        }
    }

    pub fn right_priority(&self) -> usize {
        match self {
            &BinaryOperator::Or => 1,
            &BinaryOperator::And => 2,
            &BinaryOperator::NotEqual | &BinaryOperator::Equal
            | &BinaryOperator::Less | &BinaryOperator::LessEqual
            | &BinaryOperator::Greater | &BinaryOperator::GreaterEqual => 3,
            &BinaryOperator::Concat => 4,
            &BinaryOperator::Add | &BinaryOperator::Sub => 6,
            &BinaryOperator::Mul | &BinaryOperator::Div | &BinaryOperator::Mod => 7,
            &BinaryOperator::Pow => 9,
        }
    }
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
