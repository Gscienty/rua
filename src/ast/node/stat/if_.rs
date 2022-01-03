use super::super::{super::LexLocation, AstExpr, AstStat};

#[derive(Clone)]
pub struct StatIf {
    condition: Box<AstExpr>,
    then_body: Box<AstStat>,
    else_body: Box<AstStat>,

    has_then: bool,
    then_locaction: LexLocation,

    has_else: bool,
    else_location: LexLocation,
}
