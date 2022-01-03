use super::super::{super::LexLocation, AstExpr, AstLocal, AstStat};

#[derive(Clone)]
pub struct StatForIn {
    vars: Vec<Box<AstLocal>>,
    values: Vec<Box<AstExpr>>,
    body: Box<AstStat>,

    has_in: bool,
    in_location: LexLocation,

    has_do: bool,
    do_location: LexLocation,

    has_end: bool,
}
