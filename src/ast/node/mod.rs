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

#[derive(Clone)]
pub enum AstNodePayload {
    None,

    ExprGroup(Box<AstExpr>),
    ExprConstantNil,
    ExprConstantBool(bool),
    ExprConstantNumber(f64),
    ExprConstantString(String),
    ExprLocal(ExprLocal),
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
    TypeIntersection(Box<TypeIntersection>),
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

impl AstStat {
    pub fn set_has_semicolon(&mut self, has_semicolon: bool) {
        self.has_semicolon = has_semicolon;
    }
}

#[derive(Clone)]
pub struct AstTypeList {
    types: Vec<AstType>,
    tail_type: AstTypePack,
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
            | &Self::ExprError(_) => true,
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
}

pub trait AstVisit {
    fn visit(&self) -> bool;
}
