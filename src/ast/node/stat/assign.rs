use super::super::AstExpr;

pub struct StatAssign {
    vars: Vec<Box<AstExpr>>,
    values: Vec<Box<AstExpr>>,
}
