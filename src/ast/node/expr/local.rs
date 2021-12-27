use super::super::AstLocal;

pub struct ExprLocal {
    local: Box<AstLocal>,
    upvalue: bool,
}
