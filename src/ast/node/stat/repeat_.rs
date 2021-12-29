use super::{super::AstExpr, StatBlock};

pub struct StatRepeat {
    condition: Box<AstExpr>,
    body: Box<StatBlock>,

    has_until: bool,
}
