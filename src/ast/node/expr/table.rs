use super::super::AstExpr;

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
