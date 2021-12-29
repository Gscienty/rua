mod argument_name;
mod expr;
mod local;
mod name;
mod stat;
mod state;
mod type_;
mod type_pack;

pub use argument_name::AstArgumentName;
pub use expr::*;
pub use local::AstLocal;
pub use name::AstName;
pub use stat::*;
pub use state::AstNodeState;
pub use type_::*;
pub use type_pack::*;

pub enum AstNodePayload {
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
    ExprTable(Box<ExprTable>),
    ExprTypeAssertion(Box<ExprTypeAssertion>),
    ExprUnary(Box<ExprUnary>),
    ExprError(Box<ExprError>),

    StatAssign(Box<StatAssign>),
    StatBlock(Box<StatBlock>),
    StatCompoundAssign(Box<StatCompoundAssign>),
    StatDeclareClass(Box<StatDeclareClass>),
    StatDeclareFunction(Box<StatFunction>),
    StatDeclareGlobal(Box<StatDeclareGlobal>),
    StatExpr(Box<StatExpr>),
    StatForIn(Box<StatForIn>),
    StatFunction(Box<StatFunction>),
    StatLocal(Box<StatLocal>),
    StatLocalFunction(Box<StatLocalFunction>),
    StatBreak(Box<StatBreak>),
    StatContinue(Box<StatContinue>),
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
    TypeSingletonBool(Box<TypeSingletonBool>),
    TypeSingletonString(Box<TypeSingletonString>),
    TypeTable(Box<TypeTable>),
    TypeTypeof(Box<TypeTypeof>),
    TypeUnion(Box<TypeUnion>),

    TypePackExplicit(Box<TypePackExplicit>),
    TypePackGeneric(Box<TypePackGeneric>),
    TypePackVariadic(Box<TypePackVariadic>),
}

pub struct AstNode {
    state: AstNodeState,
    payload: AstNodePayload,
}

pub type AstExpr = AstNode;
pub type AstType = AstNode;
pub type AstTypePack = AstNode;
pub type AstStat = AstNode;

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
            | &Self::StatBreak(_)
            | &Self::StatContinue(_)
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
