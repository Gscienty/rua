use super::{
    super::{super::LexLocation, AstExpr, AstStat},
    StatBlock,
};

pub struct StatWhile {
    condition: Box<AstExpr>,
    body: Box<StatBlock>,

    has_do: bool,
    do_location: LexLocation,

    has_end: bool,
}
