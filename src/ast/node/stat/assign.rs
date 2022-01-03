use super::super::AstExpr;

#[derive(Clone)]
pub struct StatAssign {
    vars: Vec<Box<AstExpr>>,
    values: Vec<Box<AstExpr>>,
}
