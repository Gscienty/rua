use super::{
    super::{AstNodePayload, AstType, LexLocation},
    TableIndexer, TableProp,
};

#[derive(Clone)]
pub struct TypeTable {
    props: Vec<TableProp>,
    indexer: Option<TableIndexer>,
}

impl TypeTable {
    pub fn new(
        location: LexLocation,
        props: Vec<TableProp>,
        indexer: Option<TableIndexer>,
    ) -> Box<AstType> {
        AstType::new(
            location,
            AstNodePayload::TypeTable(Box::new(TypeTable { props, indexer })),
        )
    }
}
