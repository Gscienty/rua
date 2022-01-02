use super::super::{LexLocation, LexType};

pub struct Comment {
    type_: LexType,
    location: LexLocation,
}

impl Comment {
    pub fn new(type_: LexType, location: LexLocation) -> Self {
        Comment { type_, location }
    }
}
