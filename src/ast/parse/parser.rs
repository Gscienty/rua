use super::super::*;
use std::collections::HashMap;

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

        AstExpr::new(
            self.lexer.get_current_location(),
            ExprError::new(Vec::new(), self.error_msgs.len() - 1),
        )
    }

    fn next_lexeme(&mut self) {
        self.lexer.next(true);
    }

    fn parse_string(&mut self) -> Result<Box<AstExpr>, Box<AstExpr>> {
        let current_lexeme = self.lexer.get_current();
        self.next_lexeme();

        match current_lexeme.get_type() {
            LexType::RawString(value) => Ok(AstExpr::new(
                current_lexeme.get_location(),
                AstNodePayload::ExprConstantString(value),
            )),
            LexType::QuotedString(value) => Ok(AstExpr::new(
                current_lexeme.get_location(),
                AstNodePayload::ExprConstantString(value),
            )),
            _ => Err(self.report_expr_error("String literal contains malformed escape sequence")),
        }
    }

    fn parse_name(&mut self, error_msg: &str) -> Result<(AstName, LexLocation), Box<AstExpr>> {
        let current_lexeme = self.lexer.get_current();
        self.next_lexeme();

        if let LexType::Name(name) = current_lexeme.get_type() {
            Ok((AstName::new(name), current_lexeme.get_location()))
        } else {
            Err(self.report_expr_error(error_msg))
        }
    }

    fn parse_number(&mut self, error_msg: &str) -> Result<Box<AstExpr>, Box<AstExpr>> {
        let current_lexeme = self.lexer.get_current();
        self.next_lexeme();

        if let LexType::Number(value) = current_lexeme.get_type() {
            let clear_value = value.replace("_", "");
            let mut clear_value_chars = clear_value.chars();

            let mut dec_fn = || {
                if let Ok(result) = clear_value.parse::<f64>() {
                    Ok(AstExpr::new(
                        current_lexeme.get_location(),
                        AstNodePayload::ExprConstantNumber(result),
                    ))
                } else {
                    Err(self.report_expr_error(error_msg))
                }
            };

            if clear_value_chars.nth(0).eq(&Some('0')) {
                match clear_value_chars.nth(1) {
                    Some('b') | Some('B') | Some('x') | Some('X') => {
                        if let Ok(result) = clear_value.parse::<u64>() {
                            Ok(AstExpr::new(
                                current_lexeme.get_location(),
                                AstNodePayload::ExprConstantNumber(result as f64),
                            ))
                        } else {
                            Err(self.report_expr_error(error_msg))
                        }
                    }
                    _ => dec_fn(),
                }
            } else {
                dec_fn()
            }
        } else {
            Err(self.report_expr_error(error_msg))
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
            if let Ok(result) = parser.parse_name("") {
                assert_eq!(result.0, AstName::new(expect.get(i).unwrap().clone()));
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
            if let Ok(result) = parser.parse_string() {
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
}
