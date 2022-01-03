use super::super::{AstName, AstType};

#[derive(Clone)]
pub struct StatDeclareGlobal {
    name: AstName,
    type_: Box<AstType>,
}
