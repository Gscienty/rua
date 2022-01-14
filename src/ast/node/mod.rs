mod argument_name;
mod expr;
mod local;
mod name;
mod stat;
mod type_;

pub use argument_name::AstArgumentName;
pub use expr::*;
pub use local::AstLocal;
pub use name::AstName;
pub use stat::*;
pub use type_::*;

#[derive(Debug, PartialEq, Eq)]
pub enum AstNodePayloadType {
    None,

    ExprGroup,
    ExprConstantNil,
    ExprConstantBool,
    ExprConstantNumber,
    ExprConstantString,
    ExprLocal,
    ExprGlobal,
    ExprVarargs,
    ExprCall,
    ExprIndexName,
    ExprIndexExpr,
    ExprBinary,
    ExprFunction,
    ExprIfElse,
    ExprTable,
    ExprTypeAssertion,
    ExprUnary,
    ExprError,

    StatAssign,
    StatBlock,
    StatCompoundAssign,
    StatDeclareClass,
    StatDeclareFunction,
    StatDeclareGlobal,
    StatExpr,
    StatForIn,
    StatFunction,
    StatLocal,
    StatLocalFunction,
    StatBreak,
    StatContinue,
    StatFor,
    StatIf,
    StatRepeat,
    StatReturn,
    StatWhile,
    StatTypeAlias,
    StatError,

    TypeError,
    TypeFunction,
    TypeIntersection,
    TypeReference,
    TypeSingletonBool,
    TypeSingletonString,
    TypeTable,
    TypeTypeof,
    TypeUnion,

    TypePackExplicit,
    TypePackGeneric,
    TypePackVariadic,
}

#[derive(Clone)]
pub enum AstNodePayload {
    None,

    ExprGroup(Box<AstExpr>),
    ExprConstantNil,
    ExprConstantBool(bool),
    ExprConstantNumber(f64),
    ExprConstantString(String),
    ExprLocal(Box<ExprLocal>),
    ExprGlobal(AstName),
    ExprVarargs,
    ExprCall(Box<ExprCall>),
    ExprIndexName(Box<ExprIndexName>),
    ExprIndexExpr(Box<ExprIndexExpr>),
    ExprBinary(Box<ExprBinary>),
    ExprFunction(Box<ExprFunction>),
    ExprIfElse(Box<ExprIfElse>),
    ExprTable(Vec<TableItem>),
    ExprTypeAssertion(Box<ExprTypeAssertion>),
    ExprUnary(Box<ExprUnary>),
    ExprError(Box<ExprError>),

    StatAssign(Box<StatAssign>),
    StatBlock(Vec<Box<AstStat>>),
    StatCompoundAssign(Box<StatCompoundAssign>),
    StatDeclareClass(Box<StatDeclareClass>),
    StatDeclareFunction(Box<StatFunction>),
    StatDeclareGlobal(Box<StatDeclareGlobal>),
    StatExpr(Box<StatExpr>),
    StatForIn(Box<StatForIn>),
    StatFunction(Box<StatFunction>),
    StatLocal(Box<StatLocal>),
    StatLocalFunction(Box<StatLocalFunction>),
    StatBreak,
    StatContinue,
    StatFor(Box<StatFor>),
    StatIf(Box<StatIf>),
    StatRepeat(Box<StatRepeat>),
    StatReturn(Box<StatReturn>),
    StatWhile(Box<StatWhile>),
    StatTypeAlias(Box<StatTypeAlias>),
    StatError(Box<StatError>),

    TypeError(Box<TypeError>),
    TypeFunction(Box<TypeFunction>),
    TypeIntersection(Vec<Box<AstType>>),
    TypeReference(Box<TypeReference>),
    TypeSingletonBool(bool),
    TypeSingletonString(String),
    TypeTable(Box<TypeTable>),
    TypeTypeof(Box<AstExpr>),
    TypeUnion(Vec<Box<AstType>>),

    TypePackExplicit(Box<AstTypeList>),
    TypePackGeneric(AstName),
    TypePackVariadic(Box<AstType>),
}

use super::LexLocation;

#[derive(Clone)]
pub struct AstNodeState {
    class_index: i32,
    location: LexLocation,
}

impl AstNodeState {
    pub fn new(class_index: i32, location: LexLocation) -> Self {
        AstNodeState {
            class_index,
            location,
        }
    }

    pub fn get_location(&self) -> LexLocation {
        self.location
    }
}

#[derive(Clone)]
pub struct AstNode {
    state: AstNodeState,
    payload: AstNodePayload,

    has_semicolon: bool,
}

impl AstNode {
    pub fn get_payload(&self) -> AstNodePayload {
        self.payload.clone()
    }

    pub fn get_location(&self) -> LexLocation {
        self.state.get_location()
    }
}

pub type AstExpr = AstNode;
pub type AstType = AstNode;
pub type AstTypePack = AstNode;
pub type AstStat = AstNode;

impl AstNode {
    pub fn new_nil() -> Box<Self> {
        Box::new(AstNode {
            state: AstNodeState::new(0, LexLocation::zero()),
            payload: AstNodePayload::None,
            has_semicolon: false,
        })
    }
    pub fn new(location: LexLocation, payload: AstNodePayload) -> Box<Self> {
        Box::new(AstNode {
            state: AstNodeState::new(0, location),
            payload,
            has_semicolon: false,
        })
    }
}

impl AstType {
    pub fn is_type_pack(&self) -> bool {
        match self.payload {
            AstNodePayload::TypePackVariadic(_)
            | AstNodePayload::TypePackGeneric(_)
            | AstNodePayload::TypePackExplicit(_) => true,
            _ => false,
        }
    }
}

impl AstStat {
    pub fn set_has_semicolon(&mut self, has_semicolon: bool) {
        self.has_semicolon = has_semicolon;
    }
}

#[derive(Clone)]
pub struct AstTypeList {
    types: Vec<AstType>,
    tail_type: Box<AstTypePack>,
}

impl AstNodePayload {
    pub fn is_expr(&self) -> bool {
        match self {
            &Self::ExprError(_)
            | &Self::ExprGroup(_)
            | &Self::ExprConstantNil
            | &Self::ExprConstantBool(_)
            | &Self::ExprConstantNumber(_)
            | &Self::ExprLocal(_)
            | &Self::ExprGlobal(_)
            | &Self::ExprVarargs
            | &Self::ExprCall(_)
            | &Self::ExprIndexName(_)
            | &Self::ExprIndexExpr(_)
            | &Self::ExprBinary(_)
            | &Self::ExprFunction(_)
            | &Self::ExprIfElse(_)
            | &Self::ExprTable(_)
            | &Self::ExprTypeAssertion(_)
            | &Self::ExprUnary(_) => true,
            _ => false,
        }
    }

    pub fn is_stat(&self) -> bool {
        match self {
            &Self::StatAssign(_)
            | &Self::StatBlock(_)
            | &Self::StatCompoundAssign(_)
            | &Self::StatDeclareClass(_)
            | &Self::StatDeclareFunction(_)
            | &Self::StatDeclareGlobal(_)
            | &Self::StatExpr(_)
            | &Self::StatForIn(_)
            | &Self::StatFunction(_)
            | &Self::StatLocal(_)
            | &Self::StatLocalFunction(_)
            | &Self::StatBreak
            | &Self::StatContinue
            | &Self::StatFor(_)
            | &Self::StatIf(_)
            | &Self::StatRepeat(_)
            | &Self::StatReturn(_)
            | &Self::StatWhile(_)
            | &Self::StatTypeAlias(_)
            | &Self::StatError(_) => true,
            _ => false,
        }
    }

    pub fn is_type(&self) -> bool {
        match self {
            &Self::TypeError(_)
            | &Self::TypeFunction(_)
            | &Self::TypeIntersection(_)
            | &Self::TypeReference(_)
            | &Self::TypeSingletonBool(_)
            | &Self::TypeSingletonString(_)
            | &Self::TypeTable(_)
            | &Self::TypeTypeof(_)
            | &Self::TypeUnion(_) => true,
            _ => false,
        }
    }

    pub fn get_type(&self) -> AstNodePayloadType {
        match self {
            &Self::ExprError(_) => AstNodePayloadType::ExprError,
            &Self::ExprGroup(_) => AstNodePayloadType::ExprGroup,
            &Self::ExprConstantNil => AstNodePayloadType::ExprConstantNil,
            &Self::ExprConstantBool(_) => AstNodePayloadType::ExprConstantBool,
            &Self::ExprConstantNumber(_) => AstNodePayloadType::ExprConstantNumber,
            &Self::ExprLocal(_) => AstNodePayloadType::ExprLocal,
            &Self::ExprGlobal(_) => AstNodePayloadType::ExprGlobal,
            &Self::ExprVarargs => AstNodePayloadType::ExprVarargs,
            &Self::ExprCall(_) => AstNodePayloadType::ExprCall,
            &Self::ExprIndexName(_) => AstNodePayloadType::ExprIndexName,
            &Self::ExprIndexExpr(_) => AstNodePayloadType::ExprIndexExpr,
            &Self::ExprBinary(_) => AstNodePayloadType::ExprBinary,
            &Self::ExprFunction(_) => AstNodePayloadType::ExprFunction,
            &Self::ExprIfElse(_) => AstNodePayloadType::ExprIfElse,
            &Self::ExprTable(_) => AstNodePayloadType::ExprTable,
            &Self::ExprTypeAssertion(_) => AstNodePayloadType::ExprTypeAssertion,
            &Self::ExprUnary(_) => AstNodePayloadType::ExprUnary,

            &Self::StatAssign(_) => AstNodePayloadType::StatAssign,
            &Self::StatBlock(_) => AstNodePayloadType::StatBlock,
            &Self::StatCompoundAssign(_) => AstNodePayloadType::StatCompoundAssign,
            &Self::StatDeclareClass(_) => AstNodePayloadType::StatDeclareClass,
            &Self::StatDeclareFunction(_) => AstNodePayloadType::StatDeclareFunction,
            &Self::StatDeclareGlobal(_) => AstNodePayloadType::StatDeclareGlobal,
            &Self::StatExpr(_) => AstNodePayloadType::StatExpr,
            &Self::StatForIn(_) => AstNodePayloadType::StatForIn,
            &Self::StatFunction(_) => AstNodePayloadType::StatFunction,
            &Self::StatLocal(_) => AstNodePayloadType::StatLocal,
            &Self::StatLocalFunction(_) => AstNodePayloadType::StatLocalFunction,
            &Self::StatBreak => AstNodePayloadType::StatBreak,
            &Self::StatContinue => AstNodePayloadType::StatContinue,
            &Self::StatFor(_) => AstNodePayloadType::StatFor,
            &Self::StatIf(_) => AstNodePayloadType::StatIf,
            &Self::StatRepeat(_) => AstNodePayloadType::StatRepeat,
            &Self::StatReturn(_) => AstNodePayloadType::StatReturn,
            &Self::StatWhile(_) => AstNodePayloadType::StatWhile,
            &Self::StatTypeAlias(_) => AstNodePayloadType::StatTypeAlias,
            &Self::StatError(_) => AstNodePayloadType::StatError,

            &Self::TypeError(_) => AstNodePayloadType::TypeError,
            &Self::TypeFunction(_) => AstNodePayloadType::TypeFunction,
            &Self::TypeIntersection(_) => AstNodePayloadType::TypeIntersection,
            &Self::TypeReference(_) => AstNodePayloadType::TypeReference,
            &Self::TypeSingletonBool(_) => AstNodePayloadType::TypeSingletonBool,
            &Self::TypeSingletonString(_) => AstNodePayloadType::TypeSingletonString,
            &Self::TypeTable(_) => AstNodePayloadType::TypeTable,
            &Self::TypeTypeof(_) => AstNodePayloadType::TypeTypeof,
            &Self::TypeUnion(_) => AstNodePayloadType::TypeUnion,

            _ => AstNodePayloadType::None,
        }
    }
}

pub trait AstVisit {
    fn visit(&self) -> bool;
}
