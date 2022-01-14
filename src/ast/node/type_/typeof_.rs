use super::super::{AstExpr, AstNodePayload, AstType, LexLocation};

pub fn new_type_typeof(location: LexLocation, expr: Box<AstExpr>) -> Box<AstType> {
    AstType::new(location, AstNodePayload::TypeTypeof(expr))
}
