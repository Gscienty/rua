use crate::ast::LexLocation;

pub struct AstNodeState {
    class_index: i32,
    location: LexLocation,
}

impl AstNodeState {
    pub fn new(class_index: i32, location: LexLocation) -> Self {
        AstNodeState {
            class_index,
            location,
        }
    }
}
