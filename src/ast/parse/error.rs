use super::super::LexLocation;

pub struct ParseError {
    location: LexLocation,
    message: String,
}
