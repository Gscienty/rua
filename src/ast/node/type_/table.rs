use super::{AstTableIndexer, AstTableProp};

#[derive(Clone)]
pub struct TypeTable {
    props: Vec<Box<AstTableProp>>,
    indexer: Box<AstTableIndexer>,
}
