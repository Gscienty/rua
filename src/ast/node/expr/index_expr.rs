use super::super::AstExpr;

#[derive(Clone)]
pub struct ExprIndexExpr {
    expr: Box<AstExpr>,
    index: Box<AstExpr>,
}
