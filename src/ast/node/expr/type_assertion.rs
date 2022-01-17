use super::super::{AstExpr, AstNodePayload, AstType, LexLocation};

#[derive(Clone)]
pub struct ExprTypeAssertion {
    expr: Box<AstExpr>,
    annotation: Box<AstType>,
}

impl ExprTypeAssertion {
    pub fn new(
        location: LexLocation,
        expr: Box<AstExpr>,
        annotation: Box<AstType>,
    ) -> Box<AstExpr> {
        AstExpr::new(
            location,
            AstNodePayload::ExprTypeAssertion(Box::new(ExprTypeAssertion { expr, annotation })),
        )
    }
}
