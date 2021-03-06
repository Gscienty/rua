use super::super::{AstArgumentName, AstName, AstTypeList};

#[derive(Clone)]
pub struct TypeFunction {
    generics: Vec<AstName>,
    generic_packs: Vec<AstName>,
    arg_types: AstTypeList,
    arg_names: Vec<Option<AstArgumentName>>,
    return_types: AstTypeList,
}
