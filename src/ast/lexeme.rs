use super::{LexLocation, LexType};
use std::fmt::Display;

#[derive(Clone)]
pub struct Lexeme {
    lex_type: LexType,
    location: LexLocation,
}

impl Lexeme {
    pub fn new(location: LexLocation, lex_type: LexType) -> Self {
        Lexeme { lex_type, location }
    }

    pub const fn get_location(&self) -> LexLocation {
        self.location
    }

    pub fn get_type(&self) -> LexType {
        self.lex_type.clone()
    }
}

impl Display for Lexeme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<{}, location: {}>",
            self.lex_type.to_string(),
            self.location
        )
    }
}
