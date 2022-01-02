use super::{super::StatBlock, Comment, ParseError};

pub struct ParseResult {
    root: Box<StatBlock>,
    hot_comments: Vec<String>,
    errors: Vec<ParseError>,

    comment_locations: Vec<Comment>,
}
