use super::super::{super::LexLocation, AstExpr};

pub struct ExprCall {
    func: Box<AstExpr>,
    args: Vec<Box<AstExpr>>,
    self_: bool,
    arg_location: LexLocation,
}
