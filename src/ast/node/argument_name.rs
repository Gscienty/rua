use super::{super::LexLocation, AstName};

#[derive(Clone)]
pub struct AstArgumentName {
    name: AstName,
    location: LexLocation,
}
