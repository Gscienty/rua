use super::super::AstExpr;

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

pub struct ExprBinary {
    operator: BinaryOperator,
    left: Box<AstExpr>,
    right: Box<AstExpr>,
}
