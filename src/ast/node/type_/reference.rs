use super::super::{AstName, AstNodePayload, AstType, LexLocation};

#[derive(Clone)]
pub struct TypeReference {
    prefix: Option<AstName>,
    name: AstName,
    parameters: Option<Vec<Box<AstType>>>,
}

impl TypeReference {
    pub fn new(
        location: LexLocation,
        prefix: Option<AstName>,
        name: AstName,
        parameters: Option<Vec<Box<AstType>>>,
    ) -> Box<AstType> {
        AstType::new(
            location,
            AstNodePayload::TypeReference(Box::new(TypeReference {
                prefix,
                name,
                parameters,
            })),
        )
    }
}
