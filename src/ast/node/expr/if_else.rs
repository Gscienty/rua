use super::super::AstExpr;

pub struct ExprIfElse {
    condition: Box<AstExpr>,
    has_then: bool,
    true_expr: Box<AstExpr>,
    has_else: bool,
    false_expr: Box<AstExpr>,
}
