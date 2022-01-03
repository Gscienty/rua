use super::{super::AstName, AstTypeOrPack};

#[derive(Clone)]
pub struct TypeReference {
    has_prefix: bool,
    has_parameter_list: bool,
    prefix: AstName,
    name: AstName,
    parameters: Vec<AstTypeOrPack>,
}
