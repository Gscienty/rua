use super::super::AstExpr;

#[derive(Clone)]
pub struct StatExpr {
    expr: Box<AstExpr>,
}
