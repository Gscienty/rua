use super::super::{AstExpr, AstStat};

#[derive(Clone)]
pub struct StatRepeat {
    condition: Box<AstExpr>,
    body: Box<AstStat>,

    has_until: bool,
}
