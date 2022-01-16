use super::super::{AstExpr, AstNodePayload, LexLocation};

pub fn new_expr_varargs(location: LexLocation) -> Box<AstExpr> {
    AstExpr::new(location, AstNodePayload::ExprVarargs)
}
