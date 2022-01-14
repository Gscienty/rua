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

use std::collections::HashMap;
pub struct Parser<'src_lf> {
    lexer: Lexer<'src_lf>,
    local_map: HashMap<AstName, AstLocal>,
    local_stack: Vec<&'src_lf AstLocal>,
    error_msgs: Vec<String>,
    function_stack: Vec<(bool, u32)>,
}

impl<'src_lf> Parser<'src_lf> {
    pub fn new(src: &'src_lf str) -> Self {
        let mut result = Parser {
            lexer: Lexer::new(src),
            local_map: HashMap::new(),
            local_stack: Vec::new(),
            error_msgs: Vec::new(),
            function_stack: Vec::new(),
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

    fn get_ahead_lexeme(&self) -> LexType {
        self.lexer.get_ahead().get_type()
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

    fn parse_string_expr(&mut self) -> Result<Box<AstExpr>, Box<AstExpr>> {
        match self.get_lexeme() {
            LexType::RawString(value) => {
                self.next_lexeme();
                Ok(new_constant_string(self.get_location(), value))
            }
            LexType::QuotedString(value) => {
                self.next_lexeme();
                Ok(new_constant_string(self.get_location(), value))
            }
            _ => Err(self.report_expr_error("String literal contains malformed escape sequence")),
        }
    }

    fn parse_nil_expr(&mut self) -> Result<Box<AstExpr>, Box<AstExpr>> {
        let location = self.get_location();

        if self.get_lexeme().eq(&LexType::Nil) {
            self.next_lexeme();

            Ok(new_constant_nil(location))
        } else {
            Err(self.report_expr_error("unexpected nil"))
        }
    }

    fn parse_bool_expr(&mut self) -> Result<Box<AstExpr>, Box<AstExpr>> {
        let location = self.get_location();

        match self.get_lexeme() {
            LexType::True => {
                self.next_lexeme();

                Ok(new_constant_bool(location, true))
            }
            LexType::False => {
                self.next_lexeme();
                Ok(new_constant_bool(location, false))
            }
            _ => Err(self.report_expr_error("unexpected bool")),
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

    fn parse_number_expr(&mut self, error_msg: &str) -> Result<Box<AstExpr>, Box<AstExpr>> {
        if let LexType::Number(value) = self.get_lexeme() {
            let location = self.get_location();
            self.next_lexeme();

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
                location,
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

    fn parse_name(&mut self) -> Result<(AstName, LexLocation), Box<AstExpr>> {
        if let LexType::Name(value) = self.get_lexeme() {
            let location = self.get_location();
            self.next_lexeme();

            Ok((AstName::new(value), location))
        } else {
            Err(self.report_expr_error("unexpected name"))
        }
    }

    fn parse_name_expr(&mut self) -> Result<Box<AstExpr>, Box<AstExpr>> {
        let (name, location) = self.parse_name()?;

        Ok(if let Some(local) = self.local_map.get(&name) {
            ExprLocal::new(
                location,
                local.clone(),
                local.get_function_depth() != self.function_stack.len() - 1,
            )
        } else {
            new_expr_global(location, name)
        })
    }

    fn parse_prefix_expr(&mut self) -> Result<Box<AstExpr>, Box<AstExpr>> {
        if self.get_lexeme() == LexType::LeftRoundBracket {
            let start = self.get_location();
            self.next_lexeme();
            let expr = self.parse_expr()?;
            let end = self.get_location();

            if self.get_lexeme() != LexType::RightRoundBracket {
                Err(self.report_expr_error("unexpected right round bracket"))
            } else {
                self.next_lexeme();

                Ok(new_expr_group(
                    LexLocation::new(start.get_begin(), end.get_end()),
                    expr,
                ))
            }
        } else {
            self.parse_name_expr()
        }
    }

    fn parse_table_constructor(&mut self) -> Result<Box<AstExpr>, Box<AstExpr>> {
        // TODO
    }

    fn parse_expr_list(&mut self, args: &mut Vec<Box<AstExpr>>) -> Result<(), Box<AstExpr>> {
        args.push(self.parse_expr()?);

        while self.get_lexeme() == LexType::Comma {
            self.next_lexeme();
            args.push(self.parse_expr()?);
        }

        Ok(())
    }

    fn parse_function_args_expr(
        &mut self,
        func: Box<AstExpr>,
        has_self: bool,
        self_location: LexLocation,
    ) -> Result<Box<AstExpr>, Box<AstExpr>> {
        Ok(match self.get_lexeme() {
            // <func>(<arg>[,<arg>])
            LexType::LeftRoundBracket => {
                let arg_start = self.get_location().get_end();
                self.next_lexeme();

                let mut args: Vec<Box<AstExpr>> = Vec::new();
                if self.get_lexeme() != LexType::RightRoundBracket {
                    self.parse_expr_list(&mut args)?;
                }
                let end = self.get_location();
                let arg_end = end.get_end();
                self.next_lexeme();

                ExprCall::new(
                    LexLocation::new(func.get_location().get_begin(), end.get_end()),
                    func,
                    args,
                    has_self,
                    LexLocation::new(arg_start, arg_end),
                )
            }
            // <func>{<table>}
            LexType::LeftCurlyBracket => {
                let arg_start = self.get_location().get_end();
                let expr = self.parse_table_constructor()?;
                let arg_end = self.get_previous_location().get_end();

                ExprCall::new(
                    LexLocation::new(
                        func.get_location().get_begin(),
                        expr.get_location().get_end(),
                    ),
                    func,
                    vec![expr],
                    has_self,
                    LexLocation::new(arg_start, arg_end),
                )
            }
            // <func>"string"
            LexType::RawString(_) | LexType::QuotedString(_) => {
                let arg_location = self.get_location();
                let expr = self.parse_string_expr()?;

                ExprCall::new(
                    LexLocation::new(
                        func.get_location().get_begin(),
                        expr.get_location().get_end(),
                    ),
                    func,
                    vec![expr],
                    has_self,
                    arg_location,
                )
            }
            _ => return Err(self.report_expr_error("unexpected function call")),
        })
    }

    fn parse_primary_expr(&mut self, as_statement: bool) -> Result<Box<AstExpr>, Box<AstExpr>> {
        let start = self.get_location();
        let mut expr = self.parse_prefix_expr()?;
        Ok(loop {
            match self.get_lexeme() {
                // <expr>.<index name>
                LexType::Dot => {
                    let op_position = self.get_location().get_begin();
                    let (index_name, index_location) = self.parse_name()?;

                    expr = ExprIndexName::new(
                        LexLocation::new(start.get_begin(), index_location.get_end()),
                        expr,
                        index_name,
                        index_location,
                        op_position,
                        '.',
                    );
                }
                // <expr>[<index_expr>]
                LexType::LeftSquareBracket => {
                    self.next_lexeme();
                    let index = self.parse_expr()?;
                    let end = self.get_location();
                    if self.get_lexeme() != LexType::RightSquareBracket {
                        return Err(self.report_expr_error("unexpected ]"));
                    }
                    self.next_lexeme();

                    expr = ExprIndexExpr::new(
                        LexLocation::new(start.get_begin(), end.get_end()),
                        expr,
                        index,
                    );
                }
                // <expr>:<index_name>
                LexType::Colon => {
                    let op_position = self.get_location().get_begin();
                    self.next_lexeme();

                    let (index, index_location) = self.parse_name()?;
                    let func = ExprIndexName::new(
                        LexLocation::new(start.get_begin(), index_location.get_end()),
                        expr,
                        index,
                        index_location,
                        op_position,
                        ':',
                    );

                    expr = self.parse_function_args_expr(func, true, index_location)?;
                }
                // <expr>(...) | {<table>} | "<string>"
                LexType::LeftRoundBracket
                | LexType::LeftCurlyBracket
                | LexType::QuotedString(_)
                | LexType::RawString(_) => {
                    expr = self.parse_function_args_expr(expr, false, LexLocation::zero())?;
                }
                _ => break expr,
            }
        })
    }

    fn parse_if_else_expr(&mut self) -> Result<Box<AstExpr>, Box<AstExpr>> {
        let mut has_else = false;
        let start = self.get_location().get_begin();
        self.next_lexeme();

        let condition = self.parse_expr()?;

        if self.get_lexeme() != LexType::Then {
            return Err(self.report_expr_error("unexpected then"));
        }
        self.next_lexeme();
        let true_expr = self.parse_expr()?;

        Ok(match self.get_lexeme() {
            LexType::ElseIf => {
                let false_expr = self.parse_if_else_expr()?;

                ExprIfElse::new(
                    LexLocation::new(start, false_expr.get_location().get_end()),
                    condition,
                    Some(true_expr),
                    Some(false_expr),
                )
            }
            LexType::Else => {
                self.next_lexeme();

                let false_expr = self.parse_expr()?;

                ExprIfElse::new(
                    LexLocation::new(start, false_expr.get_location().get_end()),
                    condition,
                    Some(true_expr),
                    Some(false_expr),
                )
            }
            LexType::End => {
                self.next_lexeme();

                ExprIfElse::new(
                    LexLocation::new(start, self.get_location().get_end()),
                    condition,
                    Some(true_expr),
                    None,
                )
            }
            _ => return Err(self.report_expr_error("unexpected if then expr")),
        })
    }

    fn parse_expr(&mut self) -> Result<Box<AstExpr>, Box<AstExpr>> {
        // TODO
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
            if let Ok(result) = parser.parse_name() {
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
            if let Ok(result) = parser.parse_string_expr() {
                if let AstNodePayload::ExprConstantString(value) = result.get_payload() {
                    assert_eq!(value, *expect.get(i).unwrap());
                } else {
                    panic!("failed");
                }
            } else {
                panic!("failed");
            }
        }
    }

    #[test]
    fn parse_number() {
        let test_fn = |t: &str, expect_value: f64| {
            let mut parser = Parser::new(t);

            if let Ok(result) = parser.parse_number_expr("") {
                if let AstNodePayload::ExprConstantNumber(value) = result.get_payload() {
                    assert_eq!(value, expect_value);
                } else {
                    panic!("failed");
                }
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

    #[test]
    fn parse_expr_bool() {
        let expect = vec![true, false];
        let mut parser = Parser::new("true false");

        for i in 0..2 {
            if let Ok(result) = parser.parse_bool_expr() {
                if let AstNodePayload::ExprConstantBool(value) = result.get_payload() {
                    assert_eq!(value, *expect.get(i).unwrap());
                } else {
                    panic!("failed");
                }
            } else {
                panic!("failed");
            }
        }
    }

    #[test]
    fn parse_expr_nil() {
        let mut parser = Parser::new("nil");
        if let Ok(result) = parser.parse_nil_expr() {
            assert_eq!(
                AstNodePayload::ExprConstantNil.get_type(),
                result.get_payload().get_type()
            )
        } else {
            panic!("failed");
        }
    }
}
