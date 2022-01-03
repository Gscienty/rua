use super::super::AstType;

#[derive(Clone)]
pub struct TypeIntersection {
    types: Vec<Box<AstType>>,
}
