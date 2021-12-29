use super::super::{AstArgumentName, AstName, AstTypeList};

pub struct StatDeclareFunction {
    name: AstName,
    generics: Vec<AstName>,
    generic_packs: Vec<AstName>,
    params: AstTypeList,
    param_names: Vec<Option<AstArgumentName>>,
    return_types: AstTypeList,
}
