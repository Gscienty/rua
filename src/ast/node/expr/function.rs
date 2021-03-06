use super::super::{super::LexLocation, AstLocal, AstName, AstStat, AstTypeList, AstTypePack};

#[derive(Clone)]
pub struct ExprFunction {
    generics: Vec<AstName>,
    generic_packs: Vec<AstName>,
    self_: Box<AstLocal>,
    args: Vec<Box<AstLocal>>,
    has_return_annotation: bool,
    return_annotation: AstTypeList,
    vararg: bool,
    vararg_location: LexLocation,
    vararg_annotation: Box<AstTypePack>,
    body: Box<AstStat>,
    function_depth: u32,
    debug_name: AstName,
    has_end: bool,
    arg_location: Option<LexLocation>,
}
