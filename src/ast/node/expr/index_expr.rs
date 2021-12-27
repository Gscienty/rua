use super::super::AstExpr;

pub struct ExprIndexExpr {
    expr: Box<AstExpr>,
    index: Box<AstExpr>,
}
