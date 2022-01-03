mod error;
mod function;
mod intersection;
mod reference;
mod table;

pub use error::TypeError;
pub use function::TypeFunction;
pub use intersection::TypeIntersection;
pub use reference::TypeReference;
pub use table::TypeTable;

use super::{super::LexLocation, AstName, AstType, AstTypePack};

#[derive(Clone)]
pub struct AstTypeOrPack {
    type_: Option<Box<AstType>>,
    type_pack: Option<Box<AstTypePack>>,
}

#[derive(Clone)]
pub struct AstTableProp {
    name: AstName,
    location: LexLocation,
    type_: Box<AstType>,
}

#[derive(Clone)]
pub struct AstTableIndexer {
    index_type: Box<AstType>,
    result_type: Box<AstType>,
    location: LexLocation,
}
