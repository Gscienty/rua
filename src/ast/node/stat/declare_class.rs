use super::super::{AstName, AstType};

#[derive(Clone)]
pub struct DeclaredClassProp {
    name: AstName,
    type_: Box<AstType>,
    is_method: bool,
}

#[derive(Clone)]
pub struct StatDeclareClass {
    name: AstName,
    super_name: Option<AstName>,
    props: Vec<DeclaredClassProp>,
}
