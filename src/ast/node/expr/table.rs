use super::super::AstExpr;

pub enum TableKind {
    List,
    Record,
    General,
}

pub struct TableItem {
    kind: TableKind,

    key: Box<AstExpr>,
    value: Box<AstExpr>,
}

pub struct ExprTable {
    items: Vec<TableItem>,
}
