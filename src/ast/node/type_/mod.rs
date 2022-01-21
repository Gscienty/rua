mod error;
mod function;
mod intersection;
mod pack;
mod reference;
mod singleton;
mod table;
mod typeof_;
mod union;

pub use error::TypeError;
pub use function::TypeFunction;
pub use intersection::*;
pub use pack::*;
pub use reference::TypeReference;
pub use singleton::*;
pub use table::TypeTable;
pub use typeof_::*;
pub use union::*;

use super::{super::LexLocation, AstName, AstType};

#[derive(Clone)]
pub struct TableProp {
    name: AstName,
    location: LexLocation,
    type_: Box<AstType>,
}

impl TableProp {
    pub fn new(name: AstName, location: LexLocation, type_: Box<AstType>) -> TableProp {
        TableProp {
            name,
            location,
            type_,
        }
    }
}

#[derive(Clone)]
pub struct TableIndexer {
    index_type: Box<AstType>,
    result_type: Box<AstType>,
    location: LexLocation,
}

impl TableIndexer {
    pub fn new(
        index_type: Box<AstType>,
        result_type: Box<AstType>,
        location: LexLocation,
    ) -> TableIndexer {
        TableIndexer {
            index_type,
            result_type,
            location,
        }
    }
}
