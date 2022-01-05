use super::super::{AstExpr, AstNodePayload, LexLocation};

pub fn new_constant_nil(location: LexLocation) -> Box<AstExpr> {
    AstExpr::new(location, AstNodePayload::ExprConstantNil)
}

pub fn new_constant_bool(location: LexLocation, value: bool) -> Box<AstExpr> {
    AstExpr::new(location, AstNodePayload::ExprConstantBool(value))
}

pub fn new_constant_number(location: LexLocation, value: f64) -> Box<AstExpr> {
    AstExpr::new(location, AstNodePayload::ExprConstantNumber(value))
}

pub fn new_constant_string(location: LexLocation, value: String) -> Box<AstExpr> {
    AstExpr::new(location, AstNodePayload::ExprConstantString(value))
}
