use super::{super::LexLocation, AstName, AstNode};

pub struct AstLocal {
    name: AstName,
    location: LexLocation,
    shadow: Box<AstLocal>,
    function_depth: u32,
    loop_depth: u32,

    annotation: Box<AstNode>,
}

impl AstLocal {
    pub fn new(
        name: AstName,
        location: LexLocation,
        shadow: Box<Self>,
        function_depth: u32,
        loop_depth: u32,
        annotation: Box<AstNode>,
    ) -> Self {
        AstLocal {
            name,
            location,
            shadow,
            function_depth,
            loop_depth,
            annotation,
        }
    }
}
