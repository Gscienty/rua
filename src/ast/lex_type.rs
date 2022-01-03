#[derive(Clone, PartialEq, Debug)]
pub enum LexType {
    Eof,
    CharEnd,
    Assign,
    Equal,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    Colon,
    LeftSquareBracket,
    RightSquareBracket,
    LeftRoundBracket,
    RightRoundBracket,
    LeftCurlyBracket,
    RightCurlyBracket,
    Comma,
    Sharp,
    Semicolon,
    NotEqual,
    Dot,
    Dot2,
    Dot3,
    SkinnyArrow,
    DoubleColon,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    Concat,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    ModAssign,
    PowAssign,
    ConcatAssign,
    RawString(String),
    QuotedString(String),
    Number(String),
    Name(String),
    Comment(String),
    BlockComment,
    BrokenString,
    BrokenComment,
    BrokenUnicode,
    Error,
    Begin,
    And,
    Break,
    Do,
    Else,
    ElseIf,
    End,
    False,
    For,
    Function,
    If,
    In,
    Local,
    Nil,
    Not,
    Or,
    Repeat,
    Return,
    Then,
    True,
    Until,
    While,
}

impl LexType {
    pub fn code(&self) -> i32 {
        match *self {
            LexType::Eof => 0,
            LexType::CharEnd => 1,
            LexType::Assign => 2,
            LexType::Equal => 3,
            LexType::Less => 4,
            LexType::Greater => 5,
            LexType::LessEqual => 6,
            LexType::GreaterEqual => 7,
            LexType::Colon => 8,
            LexType::LeftSquareBracket => 9,
            LexType::RightSquareBracket => 10,
            LexType::LeftRoundBracket => 11,
            LexType::RightRoundBracket => 12,
            LexType::LeftCurlyBracket => 13,
            LexType::RightCurlyBracket => 14,
            LexType::Comma => 15,
            LexType::Sharp => 16,
            LexType::Semicolon => 17,
            LexType::NotEqual => 18,
            LexType::Dot => 19,
            LexType::Dot2 => 20,
            LexType::Dot3 => 21,
            LexType::SkinnyArrow => 22,
            LexType::DoubleColon => 23,
            LexType::Add => 24,
            LexType::Sub => 25,
            LexType::Mul => 26,
            LexType::Div => 27,
            LexType::Mod => 28,
            LexType::Pow => 29,
            LexType::Concat => 30,
            LexType::AddAssign => 31,
            LexType::SubAssign => 32,
            LexType::MulAssign => 33,
            LexType::DivAssign => 34,
            LexType::ModAssign => 35,
            LexType::PowAssign => 36,
            LexType::ConcatAssign => 37,
            LexType::RawString(_) => 38,
            LexType::QuotedString(_) => 39,
            LexType::Number(_) => 40,
            LexType::Name(_) => 41,
            LexType::Comment(_) => 42,
            LexType::BlockComment => 43,
            LexType::BrokenString => 44,
            LexType::BrokenComment => 45,
            LexType::BrokenUnicode => 46,
            LexType::Error => 47,
            LexType::Begin => 48,
            LexType::And => 49,
            LexType::Break => 50,
            LexType::Do => 51,
            LexType::Else => 52,
            LexType::ElseIf => 53,
            LexType::End => 54,
            LexType::False => 55,
            LexType::For => 56,
            LexType::Function => 57,
            LexType::If => 58,
            LexType::In => 59,
            LexType::Local => 60,
            LexType::Nil => 61,
            LexType::Not => 62,
            LexType::Or => 63,
            LexType::Repeat => 64,
            LexType::Return => 65,
            LexType::Then => 66,
            LexType::True => 67,
            LexType::Until => 68,
            LexType::While => 69,
        }
    }

    pub const fn count() -> i32 {
        70
    }
}

impl ToString for LexType {
    fn to_string(&self) -> String {
        String::from(match self {
            LexType::Eof => "Eof",
            LexType::CharEnd => "CharEnd",
            LexType::Assign => "Assign",
            LexType::Equal => "Equal",
            LexType::Less => "Less",
            LexType::Greater => "Greater",
            LexType::LessEqual => "LessEqual",
            LexType::GreaterEqual => "GreaterEqual",
            LexType::Colon => "Colon",
            LexType::LeftSquareBracket => "LeftSquareBracket",
            LexType::RightSquareBracket => "RightSquareBracket",
            LexType::LeftRoundBracket => "LeftRoundBracket",
            LexType::RightRoundBracket => "RightSquareBracket",
            LexType::LeftCurlyBracket => "LeftCurlyBracket",
            LexType::RightCurlyBracket => "RightCurlyBracket",
            LexType::Comma => "Comma",
            LexType::Sharp => "Sharp",
            LexType::Semicolon => "Semicolon",
            LexType::NotEqual => "NotEqual",
            LexType::Dot => "Dot",
            LexType::Dot2 => "Dot2",
            LexType::Dot3 => "Dot3",
            LexType::SkinnyArrow => "SkinnyArrow",
            LexType::DoubleColon => "DoubleColon",
            LexType::Add => "Add",
            LexType::Sub => "Sub",
            LexType::Mul => "Mul",
            LexType::Div => "Div",
            LexType::Mod => "Mod",
            LexType::Pow => "Pow",
            LexType::Concat => "Concat",
            LexType::AddAssign => "AddAssign",
            LexType::SubAssign => "SubAssign",
            LexType::MulAssign => "MulAssign",
            LexType::DivAssign => "DivAssign",
            LexType::ModAssign => "ModAssign",
            LexType::PowAssign => "PowAssign",
            LexType::ConcatAssign => "ConcatAssign",
            LexType::RawString(val) => {
                let mut output = String::from("RawString: ");
                output.push_str(val.as_str());

                return output;
            }
            LexType::QuotedString(val) => {
                let mut output = String::from("QuotedString: ");
                output.push_str(val.as_str());

                return output;
            }
            LexType::Number(val) => {
                let mut output = String::from("Number: ");
                output.push_str(val.as_str());

                return output;
            }
            LexType::Name(val) => {
                let mut output = String::from("Name: ");
                output.push_str(val.as_str());

                return output;
            }
            LexType::Comment(val) => {
                let mut output = String::from("Comment: ");
                output.push_str(val.as_str());

                return output;
            }
            LexType::BlockComment => "BlockComment",
            LexType::BrokenString => "BrokenString",
            LexType::BrokenComment => "BrokenComment",
            LexType::BrokenUnicode => "BrokenUnicode",
            LexType::Error => "Error",
            LexType::Begin => "Begin",
            LexType::And => "And",
            LexType::Break => "Break",
            LexType::Do => "Do",
            LexType::Else => "Else",
            LexType::ElseIf => "ElseIf",
            LexType::End => "End",
            LexType::False => "False",
            LexType::For => "For",
            LexType::Function => "Function",
            LexType::If => "If",
            LexType::In => "In",
            LexType::Local => "Local",
            LexType::Nil => "Nil",
            LexType::Not => "Not",
            LexType::Or => "Or",
            LexType::Repeat => "Repeat",
            LexType::Return => "Return",
            LexType::Then => "Then",
            LexType::True => "True",
            LexType::Until => "Until",
            LexType::While => "While",
        })
    }
}
