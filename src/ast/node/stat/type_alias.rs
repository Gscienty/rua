use super::super::{AstName, AstType};

pub struct StatTypeAlias {
    name: AstName,
    generics: Vec<AstName>,
    generic_packs: Vec<AstName>,
    type_: Box<AstType>,
    exported: bool,
}
