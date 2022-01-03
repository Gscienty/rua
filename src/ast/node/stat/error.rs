use super::super::{AstExpr, AstStat};

#[derive(Clone)]
pub struct StatError {
    expressions: Vec<Box<AstExpr>>,
    statements: Vec<Box<AstStat>>,
    message_index: u32,
}
