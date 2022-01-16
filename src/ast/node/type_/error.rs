use super::super::{AstNodePayload, AstType, LexLocation};

#[derive(Clone)]
pub struct TypeError {
    types: Vec<Box<AstType>>,
    is_missing: bool,
    message_index: usize,
}

impl TypeError {
    pub fn new(
        location: LexLocation,
        types: Vec<Box<AstType>>,
        is_missing: bool,
        message_index: usize,
    ) -> Box<AstType> {
        AstType::new(
            location,
            AstNodePayload::TypeError(Box::new(TypeError {
                types,
                is_missing,
                message_index,
            })),
        )
    }
}
