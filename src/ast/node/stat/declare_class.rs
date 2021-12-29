use super::super::{AstName, AstType};

pub struct DeclaredClassProp {
    name: AstName,
    type_: Box<AstType>,
    is_method: bool,
}

pub struct StatDeclareClass {
    name: AstName,
    super_name: Option<AstName>,
    props: Vec<DeclaredClassProp>,
}
