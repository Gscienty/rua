use super::super::{
    super::{AstNodePayload, LexLocation, LexPosition},
    AstExpr, AstName,
};

#[derive(Clone)]
pub struct ExprIndexName {
    expr: Box<AstExpr>,
    index: AstName,
    index_location: LexLocation,
    op_position: LexPosition,
    op: char,
}

impl ExprIndexName {
    pub fn new(
        location: LexLocation,
        expr: Box<AstExpr>,
        index: AstName,
        index_location: LexLocation,
        op_position: LexPosition,
        op: char,
    ) -> Box<AstExpr> {
        AstExpr::new(
            location,
            AstNodePayload::ExprIndexName(Box::new(ExprIndexName {
                expr,
                index,
                index_location,
                op_position,
                op,
            })),
        )
    }
}
