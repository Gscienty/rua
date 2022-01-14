use super::super::{AstExpr, AstNodePayload, LexLocation};

pub fn new_expr_global(location: LexLocation, name: AstName) -> Box<AstExpr> {
    AstExpr::new(location, AstNodePayload::ExprGlobal(name))
}
