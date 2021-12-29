use super::{super::AstName, AstTypeOrPack};

pub struct TypeReference {
    has_prefix: bool,
    has_parameter_list: bool,
    prefix: AstName,
    name: AstName,
    parameters: Vec<AstTypeOrPack>,
}
