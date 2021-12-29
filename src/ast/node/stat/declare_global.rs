use super::super::{AstName, AstType};

pub struct StatDeclareGlobal {
    name: AstName,
    type_: Box<AstType>,
}
