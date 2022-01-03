use super::super::AstLocal;

#[derive(Clone)]
pub struct ExprLocal {
    local: Box<AstLocal>,
    upvalue: bool,
}
