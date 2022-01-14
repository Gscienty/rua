use super::super::{AstExpr, AstNodePayload, LexLocation};

#[derive(Clone)]
pub enum TableKind {
    List,
    Record,
    General,
}

#[derive(Clone)]
pub struct TableItem {
    kind: TableKind,

    key: Box<AstExpr>,
    value: Box<AstExpr>,
}

impl TableItem {
    pub fn new(kind: TableKind, key: Box<AstExpr>, value: Box<AstExpr>) -> Self {
        TableItem { kind, key, value }
    }
}

pub fn new_expr_table(location: LexLocation, items: Vec<TableItem>) -> Box<AstExpr> {
    AstExpr::new(location, AstNodePayload::ExprTable(items))
}
