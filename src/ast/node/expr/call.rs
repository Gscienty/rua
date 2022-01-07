use super::super::{super::LexLocation, AstExpr, AstNodePayload};

#[derive(Clone)]
pub struct ExprCall {
    func: Box<AstExpr>,
    args: Vec<Box<AstExpr>>,
    self_: bool,
    arg_location: LexLocation,
}

impl ExprCall {
    pub fn new(
        location: LexLocation,
        func: Box<AstExpr>,
        args: Vec<Box<AstExpr>>,
        self_: bool,
        arg_location: LexLocation,
    ) -> Box<AstExpr> {
        AstExpr::new(
            location,
            AstNodePayload::ExprCall(Box::new(ExprCall {
                func,
                args,
                self_,
                arg_location,
            })),
        )
    }
}
