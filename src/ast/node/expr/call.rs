use super::super::{super::LexLocation, AstExpr};

#[derive(Clone)]
pub struct ExprCall {
    func: Box<AstExpr>,
    args: Vec<Box<AstExpr>>,
    self_: bool,
    arg_location: LexLocation,
}
