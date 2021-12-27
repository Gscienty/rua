mod expr;
mod local;
mod name;
mod stat;
mod state;

pub use expr::*;
pub use local::AstLocal;
pub use name::AstName;
pub use stat::*;
pub use state::AstNodeState;

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
