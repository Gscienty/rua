use super::super::*;
use std::str::Chars;

pub struct ParseError {
    location: LexLocation,
    message: String,
}

impl ParseError {
    pub fn new(location: LexLocation, message: &str) -> Self {
        ParseError {
            location,
            message: String::from(message),
        }
    }
}

pub struct ParserOptions {
    allow_type_annotations: bool,
    support_continue_statement: bool,
    allow_declaration_syntax: bool,
    capture_comments: bool,
}

pub struct Parser<'src_lf> {
    lexer: Lexer<'src_lf>,

    error_msgs: Vec<String>,
}

impl<'src_lf> Parser<'src_lf> {
    pub fn new(src: &'src_lf str) -> Self {
        let mut result = Parser {
            lexer: Lexer::new(src),

            error_msgs: Vec::new(),
        };
        result.next_lexeme();

        result
    }

    fn report_expr_error(&mut self, error_msg: &str) -> Box<AstExpr> {
        self.error_msgs.push(String::from(error_msg));

        ExprError::new(self.get_location(), Vec::new(), self.error_msgs.len() - 1)
    }

    fn get_lexeme(&self) -> LexType {
        self.lexer.get_current_type()
    }

    fn get_location(&self) -> LexLocation {
        self.lexer.get_current_location()
    }

    fn get_previous_location(&self) -> LexLocation {
        self.lexer.get_previous_location()
    }

    fn next_lexeme(&mut self) {
        self.lexer.next(true);
    }

    fn expr_string(&mut self) -> Result<Box<AstExpr>, Box<AstExpr>> {
        match self.get_lexeme() {
            LexType::RawString(value) => Ok(new_constant_string(self.get_location(), value)),
            LexType::QuotedString(value) => Ok(new_constant_string(self.get_location(), value)),
            _ => Err(self.report_expr_error("String literal contains malformed escape sequence")),
        }
    }

    fn expr_name(&mut self, error_msg: &str) -> Result<(AstName, LexLocation), Box<AstExpr>> {
        if let LexType::Name(name) = self.get_lexeme() {
            Ok((AstName::new(name), self.get_location()))
        } else {
            Err(self.report_expr_error(error_msg))
        }
    }

    fn expr_nil(&mut self, error_msg: &str) -> Result<Box<AstExpr>, Box<AstExpr>> {
        if self.get_lexeme().eq(&LexType::Nil) {
            Ok(new_constant_nil(self.get_location()))
        } else {
            Err(self.report_expr_error(error_msg))
        }
    }

    fn expr_bool(&mut self, error_msg: &str) -> Result<Box<AstExpr>, Box<AstExpr>> {
        match self.get_lexeme() {
            LexType::True => Ok(new_constant_bool(self.get_location(), true)),
            LexType::False => Ok(new_constant_bool(self.get_location(), false)),
            _ => Err(self.report_expr_error(error_msg)),
        }
    }

    fn parse_binary<'a>(clear_value_chars: Chars<'a>) -> Result<f64, ()> {
        let mut value: i64 = 0;
        for c in clear_value_chars {
            value = (value << 1)
                + match c {
                    '0' => 0,
                    '1' => 1,
                    _ => return Err(()),
                }
        }
        Ok(value as f64)
    }

    fn parse_hex<'a>(clear_value_chars: Chars<'a>) -> Result<f64, ()> {
        let mut value: i64 = 0;

        for c in clear_value_chars {
            value = (value << 4)
                + i64::from(match c {
                    ('0'..='9') => u32::from(c) - u32::from('0'),
                    ('a'..='f') => u32::from(c) - u32::from('a') + 10,
                    ('A'..='F') => u32::from(c) - u32::from('A') + 10,
                    _ => return Err(()),
                })
        }
        Ok(value as f64)
    }

    fn expr_number(&mut self, error_msg: &str) -> Result<Box<AstExpr>, Box<AstExpr>> {
        if let LexType::Number(value) = self.get_lexeme() {
            let clear_value = value.replace("_", "");

            let dec_fn = || {
                if let Ok(result) = clear_value.parse::<f64>() {
                    Ok(result)
                } else {
                    Err(())
                }
            };

            let mut clear_value_chars = clear_value.chars();
            Ok(new_constant_number(
                self.get_location(),
                if clear_value_chars.next().eq(&Some('0')) {
                    match clear_value_chars.next() {
                        Some('b') | Some('B') => {
                            if let Ok(value) = Parser::parse_binary(clear_value_chars) {
                                value
                            } else {
                                return Err(self.report_expr_error(error_msg));
                            }
                        }
                        Some('x') | Some('X') => {
                            if let Ok(value) = Parser::parse_hex(clear_value_chars) {
                                value
                            } else {
                                return Err(self.report_expr_error(error_msg));
                            }
                        }
                        _ => {
                            if let Ok(value) = dec_fn() {
                                value
                            } else {
                                return Err(self.report_expr_error(error_msg));
                            }
                        }
                    }
                } else {
                    if let Ok(value) = dec_fn() {
                        value
                    } else {
                        return Err(self.report_expr_error(error_msg));
                    }
                },
            ))
        } else {
            Err(self.report_expr_error(error_msg))
        }
    }

    fn parse_unary_operator(lexeme: LexType) -> Result<UnaryOperator, ()> {
        Ok(match lexeme {
            LexType::Not => UnaryOperator::Not,
            LexType::Sub => UnaryOperator::Minus,
            LexType::Sharp => UnaryOperator::Len,
            _ => return Err(()),
        })
    }

    fn parse_binary_operator(lexeme: LexType) -> Result<BinaryOperator, ()> {
        Ok(match lexeme {
            LexType::Add => BinaryOperator::Add,
            LexType::Sub => BinaryOperator::Sub,
            LexType::Mul => BinaryOperator::Mul,
            LexType::Div => BinaryOperator::Div,
            LexType::Mod => BinaryOperator::Mod,
            LexType::Pow => BinaryOperator::Pow,
            LexType::Dot2 => BinaryOperator::Concat,
            LexType::NotEqual => BinaryOperator::NotEqual,
            LexType::Equal => BinaryOperator::Equal,
            LexType::Less => BinaryOperator::Less,
            LexType::LessEqual => BinaryOperator::LessEqual,
            LexType::Greater => BinaryOperator::Greater,
            LexType::GreaterEqual => BinaryOperator::GreaterEqual,
            LexType::And => BinaryOperator::And,
            LexType::Or => BinaryOperator::Or,
            _ => return Err(()),
        })
    }

    const BINARY_OPERATOR_PRIORITY: [(usize, usize); 15] = [
        // `+' `-' `*' `/' `%'
        (6, 6),
        (6, 6),
        (7, 7),
        (7, 7),
        (7, 7),
        // power and concat (right associative)
        (10, 9),
        (5, 4),
        // equality and inequality
        (3, 3),
        (3, 3),
        // order
        (3, 3),
        (3, 3),
        (3, 3),
        (3, 3),
        // logical (and/or)'`'`'`'`'`
        (2, 2),
        (1, 1),
    ];

    const UNARY_OPERATOR_PRIORITY: usize = 8;

    fn parse_expr(&mut self, limit: usize) -> Box<AstExpr> {
        let begin = self.get_location().get_begin();
        let mut expr: Box<AstExpr> = AstExpr::new_nil();

        if let Ok(unary_operator) = Parser::parse_unary_operator(self.get_lexeme()) {
            self.next_lexeme();

            let sub_expr = self.parse_expr(Parser::UNARY_OPERATOR_PRIORITY);

            expr = ExprUnary::new(
                LexLocation::new(begin, sub_expr.get_location().get_end()),
                unary_operator,
                sub_expr,
            );
        } else {
        }

        expr
    }

    fn parse_simple_expr(&mut self) -> Result<Box<AstExpr>, Box<AstExpr>> {
        // TODO
        match self.get_lexeme() {
            LexType::Nil => self.expr_nil("nil"),
            LexType::True | LexType::False => self.expr_bool("bool"),
            LexType::Number(_) => self.expr_number("number"),
            LexType::RawString(_) | LexType::QuotedString(_) => self.expr_string(),
            LexType::BrokenString => self.expr_string(),
            _ => Err(self.report_expr_error("unexpected lexeme type")),
        }
    }

    fn parse_prefix_expr(&mut self) -> Result<Box<AstExpr>, Box<AstExpr>> {
        if self.get_lexeme() == LexType::LeftRoundBracket {
            let begin = self.get_location().get_begin();

            self.next_lexeme();
            let expr = self.parse_expr(0);

            if self.get_lexeme() == LexType::RightRoundBracket {
                self.next_lexeme();

                Ok(new_expr_group(
                    LexLocation::new(begin, self.get_location().get_end()),
                    expr,
                ))
            } else {
                Err(self.report_expr_error("unexpected ')'"))
            }
        } else {
            Err(self.report_expr_error("expression"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_name() {
        let expect = vec![String::from("name_1"), String::from("name_2")];
        let mut parser = Parser::new("name_1 name_2");

        for i in 0..2 {
            if let Ok(result) = parser.expr_name("") {
                assert_eq!(result.0, AstName::new(expect.get(i).unwrap().clone()));

                parser.next_lexeme();
            } else {
                panic!("failed");
            }
        }
    }

    #[test]
    fn parse_string() {
        let expect = vec![String::from("foo"), String::from("bar")];

        let mut parser = Parser::new("\"foo\"   'bar'");
        for i in 0..2 {
            if let Ok(result) = parser.expr_string() {
                if let AstNodePayload::ExprConstantString(value) = result.get_payload() {
                    assert_eq!(value, *expect.get(i).unwrap());
                } else {
                    panic!("failed");
                }

                parser.next_lexeme();
            } else {
                panic!("failed");
            }
        }
    }

    #[test]
    fn parse_number() {
        let test_fn = |t: &str, expect_value: f64| {
            let mut parser = Parser::new(t);

            if let Ok(result) = parser.expr_number("") {
                if let AstNodePayload::ExprConstantNumber(value) = result.get_payload() {
                    assert_eq!(value, expect_value);
                } else {
                    panic!("failed");
                }

                parser.next_lexeme();
            } else {
                panic!("failed");
            };
        };

        test_fn("0b11001110", 0xCE as f64);
        test_fn("0B10010010", 0x92 as f64);
        test_fn("0b_1100_1110", 0xCE as f64);
        test_fn("0xAC", 0xAC as f64);
        test_fn("  0XACDE 0x001", 0xACDE as f64);
        test_fn(".32", 0.32_f64);
        test_fn("0.32_33", 0.3233_f64);
        test_fn("103_4.32_33", 1034.3233_f64);
    }
}
