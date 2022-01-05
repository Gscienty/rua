use super::super::{AstExpr, AstNodePayload, LexLocation};

#[derive(Clone)]
pub struct ExprError {
    expressions: Vec<Box<AstExpr>>,
    message_index: usize,
}

impl ExprError {
    pub fn new(
        location: LexLocation,
        expressions: Vec<Box<AstExpr>>,
        message_index: usize,
    ) -> Box<AstExpr> {
        AstExpr::new(
            location,
            AstNodePayload::ExprError(Box::new(ExprError {
                expressions,
                message_index,
            })),
        )
    }
}
