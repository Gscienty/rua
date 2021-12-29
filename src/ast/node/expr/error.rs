use super::super::AstExpr;

pub struct ExprError {
    expressions: Vec<Box<AstExpr>>,
    message_index: u32,
}
