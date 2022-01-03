use super::super::{super::LexLocation, AstExpr, AstStat};

#[derive(Clone)]
pub struct StatWhile {
    condition: Box<AstExpr>,
    body: Box<AstStat>,

    has_do: bool,
    do_location: LexLocation,

    has_end: bool,
}
