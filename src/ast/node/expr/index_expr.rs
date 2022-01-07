use super::super::{AstExpr, AstNodePayload, LexLocation};

#[derive(Clone)]
pub struct ExprIndexExpr {
    expr: Box<AstExpr>,
    index: Box<AstExpr>,
}

impl ExprIndexExpr {
    pub fn new(location: LexLocation, expr: Box<AstExpr>, index: Box<AstExpr>) -> Box<AstExpr> {
        AstExpr::new(
            location,
            AstNodePayload::ExprIndexExpr(Box::new(ExprIndexExpr { expr, index })),
        )
    }
}
