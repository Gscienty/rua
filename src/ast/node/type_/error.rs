use super::super::AstType;

#[derive(Clone)]
pub struct TypeError {
    types: Vec<Box<AstType>>,
    is_missing: bool,
    message_index: u32,
}
