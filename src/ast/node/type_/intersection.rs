use super::super::{AstNodePayload, AstType, LexLocation};

pub fn new_type_intersection(location: LexLocation, type_: Vec<Box<AstType>>) -> Box<AstType> {
    AstType::new(location, AstNodePayload::TypeIntersection(type_))
}
