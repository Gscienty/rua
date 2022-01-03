use super::super::{super::LexLocation, AstExpr, AstLocal};

#[derive(Clone)]
pub struct StatLocal {
    vars: Vec<Box<AstLocal>>,
    values: Vec<Box<AstExpr>>,

    has_equals_sign: bool,
    equals_sign_location: LexLocation,
}
