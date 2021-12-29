use super::{
    super::{super::LexLocation, AstExpr, AstLocal},
    StatBlock,
};

pub struct StatFor {
    var: Box<AstLocal>,
    from: Box<AstExpr>,
    to: Box<AstExpr>,
    step: Box<AstExpr>,
    body: Box<StatBlock>,

    has_do: bool,
    do_location: LexLocation,

    has_end: bool,
}
