use super::super::{AstName, AstNodePayload, AstType, AstTypeList, LexLocation};

pub fn new_type_pack_generic(location: LexLocation, name: AstName) -> Box<AstType> {
    AstType::new(location, AstNodePayload::TypePackGeneric(name))
}

pub fn new_type_pack_explicit(location: LexLocation, type_list: Box<AstTypeList>) -> Box<AstType> {
    AstType::new(location, AstNodePayload::TypePackExplicit(type_list))
}

pub fn new_type_pack_variadic(location: LexLocation, type_: Box<AstType>) -> Box<AstType> {
    AstType::new(location, AstNodePayload::TypePackVariadic(type_))
}
