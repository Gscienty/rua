use super::super::{AstNodePayload, AstType, LexLocation};

pub fn new_type_singleton_bool(location: LexLocation, value: bool) -> Box<AstType> {
    AstType::new(location, AstNodePayload::TypeSingletonBool(value))
}

pub fn new_type_singleton_string(location: LexLocation, value: String) -> Box<AstType> {
    AstType::new(location, AstNodePayload::TypeSingletonString(value))
}
