use super::{
    super::{super::LexLocation, AstExpr, AstLocal},
    StatBlock,
};

pub struct StatForIn {
    vars: Vec<Box<AstLocal>>,
    values: Vec<Box<AstExpr>>,
    body: Box<StatBlock>,

    has_in: bool,
    in_location: LexLocation,

    has_do: bool,
    do_location: LexLocation,

    has_end: bool,
}
