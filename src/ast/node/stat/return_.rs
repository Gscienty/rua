use super::super::AstExpr;

#[derive(Clone)]
pub struct StatReturn {
    list: Vec<Box<AstExpr>>,
}
