mod error;
mod function;
mod intersection;
mod reference;
mod singleton_bool;
mod singleton_string;
mod table;
mod typeof_;
mod union;

pub use error::TypeError;
pub use function::TypeFunction;
pub use intersection::TypeIntersection;
pub use reference::TypeReference;
pub use singleton_bool::TypeSingletonBool;
pub use singleton_string::TypeSingletonString;
pub use table::TypeTable;
pub use typeof_::TypeTypeof;
pub use union::TypeUnion;

use super::{super::LexLocation, AstName, AstType, AstTypePack};

pub struct AstTypeOrPack {
    type_: Option<Box<AstType>>,
    type_pack: Option<Box<AstTypePack>>,
}

pub struct AstTableProp {
    name: AstName,
    location: LexLocation,
    type_: Box<AstType>,
}

pub struct AstTableIndexer {
    index_type: Box<AstType>,
    result_type: Box<AstType>,
    location: LexLocation,
}
