use super::{LexLocation, LexType};

#[derive(Clone)]
pub struct Lexeme {
    lex_type: LexType,
    location: LexLocation,
}

impl Lexeme {
    pub fn new(location: LexLocation, lex_type: LexType) -> Self {
        Lexeme { lex_type, location }
    }

    pub fn get_location(&self) -> LexLocation {
        self.location
    }

    pub fn get_type(&self) -> LexType {
        self.lex_type.clone()
    }
}
