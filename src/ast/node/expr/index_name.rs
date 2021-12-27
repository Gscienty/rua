use super::super::{
    super::{LexLocation, LexPosition},
    AstExpr, AstName,
};

pub struct ExprIndexName {
    expr: Box<AstExpr>,
    index: AstName,
    index_location: LexLocation,
    op_position: LexPosition,
    op: char,
}
