use super::super::{AstExpr, AstNodePayload, LexLocation};

pub fn new_expr_group(location: LexLocation, value: Box<AstExpr>) -> Box<AstExpr> {
    AstExpr::new(location, AstNodePayload::ExprGroup(value))
}
