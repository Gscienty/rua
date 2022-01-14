use super::super::{AstExpr, AstLocal, AstNodePayload, LexLocation};

#[derive(Clone)]
pub struct ExprLocal {
    local: AstLocal,
    upvalue: bool,
}

impl ExprLocal {
    pub fn new(location: LexLocation, local: AstLocal, upvalue: bool) -> Box<AstExpr> {
        AstExpr::new(
            location,
            AstNodePayload::ExprLocal(Box::new(ExprLocal { local, upvalue })),
        )
    }
}
