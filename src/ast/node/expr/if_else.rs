use super::super::{AstExpr, AstNodePayload, LexLocation};

#[derive(Clone)]
pub struct ExprIfElse {
    condition: Box<AstExpr>,
    true_expr: Option<Box<AstExpr>>,
    false_expr: Option<Box<AstExpr>>,
}

impl ExprIfElse {
    pub fn new(
        location: LexLocation,
        condition: Box<AstExpr>,
        true_expr: Option<Box<AstExpr>>,
        false_expr: Option<Box<AstExpr>>,
    ) -> Box<AstExpr> {
        AstExpr::new(
            location,
            AstNodePayload::ExprIfElse(Box::new(ExprIfElse {
                condition,
                true_expr,
                false_expr,
            })),
        )
    }
}
