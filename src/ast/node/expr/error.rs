use super::super::{AstExpr, AstNodePayload};

#[derive(Clone)]
pub struct ExprError {
    expressions: Vec<Box<AstExpr>>,
    message_index: usize,
}

impl ExprError {
    pub fn new(expressions: Vec<Box<AstExpr>>, message_index: usize) -> AstNodePayload {
        AstNodePayload::ExprError(Box::new(ExprError {
            expressions,
            message_index,
        }))
    }
}
