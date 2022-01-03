use super::super::{AstExpr, AstNodePayload};

#[derive(Clone)]
pub struct ExprIfElse {
    condition: Box<AstExpr>,
    has_then: bool,
    true_expr: Box<AstExpr>,
    has_else: bool,
    false_expr: Box<AstExpr>,
}

impl ExprIfElse {
    pub fn new_only_then(condition: Box<AstExpr>, true_expr: Box<AstExpr>) -> AstNodePayload {
        AstNodePayload::ExprIfElse(Box::new(ExprIfElse {
            condition,
            has_then: true,
            true_expr,
            has_else: false,
            false_expr: AstExpr::new_nil(),
        }))
    }

    pub fn new(
        condition: Box<AstExpr>,
        true_expr: Box<AstExpr>,
        false_expr: Box<AstExpr>,
    ) -> AstNodePayload {
        AstNodePayload::ExprIfElse(Box::new(ExprIfElse {
            condition,
            has_then: true,
            true_expr,
            has_else: true,
            false_expr,
        }))
    }
}
