use super::{AstTableIndexer, AstTableProp};

pub struct TypeTable {
    props: Vec<Box<AstTableProp>>,
    indexer: Box<AstTableIndexer>,
}
