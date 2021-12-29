use super::super::AstType;

pub struct TypeError {
    types: Vec<Box<AstType>>,
    is_missing: bool,
    message_index: u32,
}
