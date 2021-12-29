use super::super::AstType;

pub struct TypeUnion {
    types: Vec<Box<AstType>>,
}
