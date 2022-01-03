use super::{super::LexLocation, AstName, AstNode};

#[derive(Clone)]
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

    pub fn get_name(&self) -> AstName {
        self.name.clone()
    }

    pub fn get_shadow(&self) -> Box<AstLocal> {
        self.shadow.clone()
    }
}
