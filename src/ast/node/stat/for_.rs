use super::super::{super::LexLocation, AstExpr, AstLocal, AstStat};

#[derive(Clone)]
pub struct StatFor {
    var: Box<AstLocal>,
    from: Box<AstExpr>,
    to: Box<AstExpr>,
    step: Box<AstExpr>,
    body: Box<AstStat>,

    has_do: bool,
    do_location: LexLocation,

    has_end: bool,
}
