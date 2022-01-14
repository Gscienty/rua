use super::{super::LexLocation, AstName, AstNode};

#[derive(Clone)]
pub struct AstLocal {
    name: AstName,
    location: LexLocation,
    function_depth: usize,
    loop_depth: u32,

    annotation: Box<AstNode>,
}

impl AstLocal {
    pub fn new(
        name: AstName,
        location: LexLocation,
        function_depth: usize,
        loop_depth: u32,
        annotation: Box<AstNode>,
    ) -> Self {
        AstLocal {
            name,
            location,
            function_depth,
            loop_depth,
            annotation,
        }
    }

    pub fn get_name(&self) -> AstName {
        self.name.clone()
    }

    pub fn get_function_depth(&self) -> usize {
        self.function_depth
    }
}
