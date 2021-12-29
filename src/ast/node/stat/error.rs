use super::super::{AstExpr, AstStat};

pub struct StatError {
    expressions: Vec<Box<AstExpr>>,
    statements: Vec<Box<AstStat>>,
    message_index: u32,
}
