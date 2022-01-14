use super::super::{AstNodePayload, AstType, LexLocation};

pub fn new_type_union(location: LexLocation, type_: Vec<Box<AstType>>) -> Box<AstType> {
    AstType::new(location, AstNodePayload::TypeUnion(type_))
}
