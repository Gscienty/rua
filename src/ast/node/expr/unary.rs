use super::super::AstExpr;

pub enum UnaryOperator {
    Not,
    Minus,
    Len,
}

pub struct ExprUnary {
    operator: UnaryOperator,
    expr: Box<AstExpr>,
}
