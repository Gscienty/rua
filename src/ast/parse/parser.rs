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

    fn report_type_error(&mut self, is_mission: bool, error_msg: &str) -> Box<AstType> {
        self.error_msgs.push(String::from(error_msg));

        TypeError::new(
            self.get_location(),
            Vec::new(),
            is_mission,
            self.error_msgs.len() - 1,
        )
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
            self.next_lexeme();

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
            _ => {
                self.next_lexeme();

                Err(self.report_expr_error("unexpected bool"))
            }
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

    fn parse_number_expr(&mut self) -> Result<Box<AstExpr>, Box<AstExpr>> {
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
                                return Err(self.report_expr_error("unexpected binary number"));
                            }
                        }
                        Some('x') | Some('X') => {
                            if let Ok(value) = Parser::parse_hex(clear_value_chars) {
                                value
                            } else {
                                return Err(self.report_expr_error("unexpected hex number"));
                            }
                        }
                        _ => {
                            if let Ok(value) = dec_fn() {
                                value
                            } else {
                                return Err(self.report_expr_error("unexpected dec number"));
                            }
                        }
                    }
                } else {
                    if let Ok(value) = dec_fn() {
                        value
                    } else {
                        return Err(self.report_expr_error("unexpected dec number"));
                    }
                },
            ))
        } else {
            Err(self.report_expr_error("unexpected number"))
        }
    }

    fn parse_unary_operator(&self, lexeme: LexType) -> Option<UnaryOperator> {
        Some(match lexeme {
            LexType::Not => UnaryOperator::Not,
            LexType::Sub => UnaryOperator::Minus,
            LexType::Sharp => UnaryOperator::Len,
            _ => return None,
        })
    }

    fn parse_binary_operator(&self, lexeme: LexType) -> Option<BinaryOperator> {
        Some(match lexeme {
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
            _ => return None,
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
                local
                    .get_function_depth()
                    .ne(&(self.function_stack.len() - 1)),
            )
        } else {
            new_expr_global(location, name)
        })
    }

    fn parse_prefix_expr(&mut self) -> Result<Box<AstExpr>, Box<AstExpr>> {
        if self.get_lexeme().eq(&LexType::LeftRoundBracket) {
            let start = self.get_location();
            self.next_lexeme();
            let expr = self.parse_expr(0)?;
            let end = self.get_location();

            if self.get_lexeme().ne(&LexType::RightRoundBracket) {
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
        let mut items: Vec<TableItem> = Vec::new();
        let start = self.get_location().get_begin();

        if self.get_lexeme().ne(&LexType::LeftCurlyBracket) {
            return Err(self.report_expr_error("unexpected table constructor"));
        }
        self.next_lexeme();

        while self.get_lexeme().ne(&LexType::RightCurlyBracket) {
            match self.get_lexeme() {
                LexType::LeftSquareBracket => {
                    self.next_lexeme();

                    let key = self.parse_expr(0)?;

                    if self.get_lexeme().ne(&LexType::RightSquareBracket) {
                        return Err(self.report_expr_error("unexpected table constructor"));
                    }
                    self.next_lexeme();

                    if self.get_lexeme().ne(&LexType::Assign) {
                        return Err(self.report_expr_error("unexpected table constructor"));
                    }
                    self.next_lexeme();

                    let value = self.parse_expr(0)?;

                    items.push(TableItem::new(TableKind::General, key, value));
                }
                LexType::Name(_) => {
                    if self.get_ahead_lexeme().eq(&LexType::Assign) {
                        let (name, name_location) = self.parse_name()?;

                        // skip '='
                        self.next_lexeme();

                        let key = new_constant_string(name_location, name.get_value());
                        let value = self.parse_expr(0)?;

                        items.push(TableItem::new(TableKind::Record, key, value));
                    } else {
                        let expr = self.parse_expr(0)?;

                        items.push(TableItem::new(TableKind::List, AstExpr::new_nil(), expr));
                    }
                }
                _ => {
                    let expr = self.parse_expr(0)?;

                    items.push(TableItem::new(TableKind::List, AstExpr::new_nil(), expr));
                }
            }

            if self.get_lexeme().eq(&LexType::Comma) || self.get_lexeme().eq(&LexType::Semicolon) {
                self.next_lexeme();
            }
        }

        if self.get_lexeme().ne(&LexType::RightCurlyBracket) {
            return Err(self.report_expr_error("unexpected table constructor"));
        }
        self.next_lexeme();

        let end = self.get_location().get_end();

        Ok(new_expr_table(LexLocation::new(start, end), items))
    }

    fn parse_expr_list(&mut self, args: &mut Vec<Box<AstExpr>>) -> Result<(), Box<AstExpr>> {
        args.push(self.parse_expr(0)?);

        while self.get_lexeme().eq(&LexType::Comma) {
            self.next_lexeme();
            args.push(self.parse_expr(0)?);
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
                if self.get_lexeme().ne(&LexType::RightRoundBracket) {
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
                    let index = self.parse_expr(0)?;
                    let end = self.get_location();
                    if self.get_lexeme().ne(&LexType::RightSquareBracket) {
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

        let condition = self.parse_expr(0)?;

        if self.get_lexeme().ne(&LexType::Then) {
            return Err(self.report_expr_error("unexpected then"));
        }
        self.next_lexeme();
        let true_expr = self.parse_expr(0)?;

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

                let false_expr = self.parse_expr(0)?;

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

    fn parse_simple_expr(&mut self) -> Result<Box<AstExpr>, Box<AstExpr>> {
        let start = self.get_location();

        match self.get_lexeme() {
            LexType::Nil => self.parse_nil_expr(),
            LexType::True | LexType::False => self.parse_bool_expr(),
            LexType::QuotedString(_) | LexType::RawString(_) | LexType::BrokenString => {
                self.parse_string_expr()
            }
            LexType::Number(_) => self.parse_number_expr(),
            // TODO impl function
            LexType::Function => Err(self.report_expr_error("todo")),
            LexType::Dot3 => {
                self.next_lexeme();

                if let Some(function_stack) = self.function_stack.last() {
                    Ok(new_expr_varargs(start))
                } else {
                    Err(self.report_expr_error("unexpected ..."))
                }
            }
            LexType::LeftCurlyBracket => self.parse_table_constructor(),
            LexType::If => self.parse_if_else_expr(),
            _ => self.parse_primary_expr(false),
        }
    }

    fn parse_assertion_expr(&mut self) -> Result<Box<AstExpr>, Box<AstExpr>> {
        let begin = self.get_location().get_begin();
        let expr = self.parse_simple_expr()?;

        Ok(if self.get_lexeme().eq(&LexType::DoubleColon) {
            self.next_lexeme();

            let annotation = self.parse_type_annotation()?;

            ExprTypeAssertion::new(
                LexLocation::new(begin, annotation.get_location().get_end()),
                expr,
                annotation,
            )
        } else {
            expr
        })
    }

    fn parse_expr(&mut self, limit: usize) -> Result<Box<AstExpr>, Box<AstExpr>> {
        let start = self.get_location();

        let mut expr = if let Some(operator) = self.parse_unary_operator(self.get_lexeme()) {
            self.next_lexeme();
            let sub_expr = self.parse_expr(8)?;

            ExprUnary::new(
                LexLocation::new(start.get_begin(), sub_expr.get_location().get_end()),
                operator,
                sub_expr,
            )
        } else {
            self.parse_assertion_expr()?
        };

        let mut operator = self.parse_binary_operator(self.get_lexeme());
        while operator.is_some() {
            self.next_lexeme();

            let next = self.parse_expr(limit)?;
            expr = ExprBinary::new(
                LexLocation::new(start.get_begin(), next.get_location().get_end()),
                operator.unwrap(),
                expr,
                next,
            );

            operator = self.parse_binary_operator(self.get_lexeme());
        }

        Ok(expr)
    }

    fn parse_nil_type(&mut self) -> Result<Box<AstType>, Box<AstType>> {
        let location = self.get_location();

        if self.get_lexeme().eq(&LexType::Nil) {
            self.next_lexeme();

            Ok(TypeReference::new(
                location,
                None,
                AstName::new(String::from("nil")),
                None,
            ))
        } else {
            Err(self.report_type_error(true, "unexpected nil type reference"))
        }
    }

    fn parse_bool_type(&mut self) -> Result<Box<AstType>, Box<AstType>> {
        let location = self.get_location();

        match self.get_lexeme() {
            LexType::True => {
                self.next_lexeme();

                Ok(new_type_singleton_bool(location, true))
            }
            LexType::False => {
                self.next_lexeme();

                Ok(new_type_singleton_bool(location, false))
            }
            _ => {
                self.next_lexeme();

                Err(self.report_type_error(true, "unexpected type singleton bool"))
            }
        }
    }

    fn parse_string_type(&mut self) -> Result<Box<AstType>, Box<AstType>> {
        let location = self.get_location();

        match self.get_lexeme() {
            LexType::RawString(value) => {
                self.next_lexeme();

                Ok(new_type_singleton_string(location, value))
            }
            LexType::QuotedString(value) => {
                self.next_lexeme();

                Ok(new_type_singleton_string(location, value))
            }
            LexType::BrokenString => {
                self.next_lexeme();

                Err(self.report_type_error(false, "unexpected type singleton string broken string"))
            }
            _ => {
                self.next_lexeme();

                Err(self.report_type_error(true, "unexpected type singleton string"))
            }
        }
    }

    fn parse_typeof_type(&mut self, begin: LexPosition) -> Result<Box<AstType>, Box<AstType>> {
        if self.get_lexeme().ne(&LexType::LeftRoundBracket) {
            return Err(self.report_type_error(false, "unexpected typeof"));
        }
        self.next_lexeme();

        let expr = self.parse_expr(0)?;
        let end = self.get_location().get_end();

        if self.get_lexeme().ne(&LexType::RightRoundBracket) {
            return Err(self.report_type_error(false, "unexpected typeof"));
        }
        self.next_lexeme();

        Ok(new_type_typeof(LexLocation::new(begin, end), expr))
    }

    fn should_parse_type_pack_annotation(&self) -> bool {
        match self.get_lexeme() {
            LexType::Dot3 => true,
            LexType::Name(_) => self.get_ahead_lexeme().eq(&LexType::Dot3),
            _ => false,
        }
    }

    fn parse_type_annotation(&mut self) -> Result<Box<AstType>, Box<AstType>> {
        let begin = self.get_location().get_begin();
        let mut parts: Vec<Box<AstType>> = Vec::new();
        if let Some(value) = self.parse_simple_type_annotation(false)?.0 {
            parts.push(value);
        } else {
            return Err(self.report_type_error(false, "unexpected type annotation"));
        }

        self.parse_type_annotation_parts(&mut parts, begin)
    }

    fn parse_type_annotation_parts(
        &mut self,
        parts: &mut Vec<Box<AstType>>,
        begin: LexPosition,
    ) -> Result<Box<AstType>, Box<AstType>> {
        let mut is_union = false;
        let mut is_intersection = false;
        loop {
            match self.get_lexeme() {
                LexType::SingletonOr => {
                    self.next_lexeme();

                    if let Some(value) = self.parse_simple_type_annotation(false)?.0 {
                        parts.push(value);
                    } else {
                        return Err(self.report_type_error(false, "unexpected type annotation"));
                    }
                    is_union = true;
                }
                LexType::QuestionMark => {
                    let location = self.get_location();

                    self.next_lexeme();
                    parts.push(TypeReference::new(
                        location,
                        None,
                        AstName::new(String::from("nil")),
                        None,
                    ));
                    is_union = true;
                }
                LexType::SingletonAnd => {
                    self.next_lexeme();

                    if let Some(value) = self.parse_simple_type_annotation(false)?.0 {
                        parts.push(value);
                    } else {
                        return Err(self.report_type_error(false, "unexpected type annotation"));
                    }
                    is_intersection = true;
                }
                _ => break,
            }
        }

        if parts.len().eq(&1) {
            Ok(*parts.first().unwrap())
        } else {
            let end = self.get_location().get_end();
            if is_union && is_intersection {
                Err(self
                    .report_type_error(false, "mixing union and intersection types is not allowed"))
            } else if is_union {
                Ok(new_type_union(LexLocation::new(begin, end), *parts))
            } else if is_intersection {
                Ok(new_type_intersection(LexLocation::new(begin, end), *parts))
            } else {
                Err(self
                    .report_type_error(false, "composite type was not an intersection or union"))
            }
        }
    }

    fn parse_type_pack_annotation(&mut self) -> Result<Box<AstTypePack>, Box<AstTypePack>> {
        Ok(if self.get_lexeme().eq(&LexType::Dot3) {
            let begin = self.get_location().get_begin();
            self.next_lexeme();
            let vararg = self.parse_type_annotation()?;

            new_type_pack_variadic(
                LexLocation::new(begin, vararg.get_location().get_end()),
                vararg,
            )
        } else {
            match self.get_lexeme() {
                LexType::Name(_) => {
                    if self.get_ahead_lexeme().eq(&LexType::Dot3) {
                        let (name, name_location) = self.parse_name()?;

                        self.next_lexeme();

                        new_type_pack_generic(name_location, name)
                    } else {
                        AstTypePack::new_nil()
                    }
                }
                _ => AstTypePack::new_nil(),
            }
        })
    }

    fn parse_type_or_pack_annotation(&mut self) -> Result<Box<AstType>, Box<AstType>> {
        let begin = self.get_location().get_begin();

        let type_or_pack = self.parse_simple_type_annotation(true)?;

        if let Some(type_pack) = type_or_pack.1 {
            Ok(type_pack)
        } else if let Some(type_) = type_or_pack.0 {
            let parts: Vec<Box<AstType>> = Vec::new();
            parts.push(type_);

            self.parse_type_annotation_parts(&mut parts, begin)
        } else {
            Err(self.report_type_error(true, "unexpected simple type annotation"))
        }
    }

    fn parse_type_parameters(&mut self) -> Result<Option<Vec<Box<AstTypePack>>>, Box<AstType>> {
        let parameters: Vec<Box<AstTypePack>> = Vec::new();

        if self.get_lexeme().eq(&LexType::Less) {
            let begin = self.get_location().get_begin();
            self.next_lexeme();

            loop {
                if self.should_parse_type_pack_annotation() {
                    parameters.push(self.parse_type_pack_annotation()?);
                } else if self.get_lexeme().eq(&LexType::LeftRoundBracket) {
                    parameters.push(self.parse_type_or_pack_annotation()?);
                } else if self.get_lexeme().eq(&LexType::Greater) && parameters.is_empty() {
                    break;
                } else {
                    parameters.push(self.parse_type_annotation()?);
                }

                if self.get_lexeme().eq(&LexType::Comma) {
                    self.next_lexeme();
                } else {
                    break;
                }
            }

            if self.get_lexeme().ne(&LexType::Greater) {
                return Err(self.report_type_error(false, "unexpected type parameters"));
            }
            self.next_lexeme();
        }

        Ok(if parameters.is_empty() {
            None
        } else {
            Some(parameters)
        })
    }

    fn parse_name_type(
        &mut self,
        begin: LexPosition,
        name: AstName,
    ) -> Result<Box<AstType>, Box<AstType>> {
        let mut real_name = name;
        let mut prefix: Option<AstName> = None;

        if self.get_lexeme().eq(&LexType::Dot) {
            self.next_lexeme();

            prefix = Some(name);
            let (sub_name, _) = self.parse_name()?;
            real_name = sub_name;
        }

        let parameters = self.parse_type_parameters()?;

        let end = self.get_location().get_end();
        Ok(TypeReference::new(
            LexLocation::new(begin, end),
            prefix,
            real_name,
            parameters,
        ))
    }

    fn parse_name_or_typeof_type(&mut self) -> Result<Box<AstType>, Box<AstType>> {
        let begin = self.get_location().get_begin();
        let prefix: Option<AstName> = None;
        let (name, _) = self.parse_name()?;

        if name.eq_str("typeof") {
            self.parse_typeof_type(begin)
        } else {
            self.parse_name_type(begin, name)
        }
    }

    fn parse_type_indexer_annotation(&mut self) -> Result<TableIndexer, Box<AstType>> {
        let begin = self.get_location().get_begin();
        self.next_lexeme();

        let index = self.parse_type_annotation()?;

        if self.get_lexeme().ne(&LexType::RightSquareBracket) {
            return Err(self.report_type_error(false, "unexpected parse type indexer annotation"));
        }
        self.next_lexeme();

        if self.get_lexeme().ne(&LexType::Colon) {
            return Err(self.report_type_error(false, "unexpected parse type indexer annotation"));
        }
        self.next_lexeme();

        let result = self.parse_type_annotation()?;

        Ok(TableIndexer::new(
            index,
            result,
            LexLocation::new(begin, result.get_location().get_end()),
        ))
    }

    fn parse_table_type_annotation(&mut self) -> Result<Box<AstType>, Box<AstType>> {
        let mut props: Vec<TableProp> = Vec::new();
        let mut indexer: Option<TableIndexer> = None;

        let begin = self.get_location().get_begin();

        if self.get_lexeme().ne(&LexType::LeftCurlyBracket) {
            return Err(self.report_type_error(false, "unexpected parse table type annotation"));
        }
        self.next_lexeme();

        while self.get_lexeme().ne(&LexType::RightCurlyBracket) {
            let parse_name_props_append = || -> Result<(), Box<AstType>> {
                let (name, name_location) = self.parse_name()?;

                if self.get_lexeme().ne(&LexType::Colon) {
                    return Err(self.report_type_error(false, "table field"));
                }
                self.next_lexeme();

                let type_ = self.parse_type_annotation()?;

                props.push(TableProp::new(name, name_location, type_));

                Ok(())
            };

            if self.get_lexeme().eq(&LexType::LeftSquareBracket) {
                let parse_string_props_append = |value: String| -> Result<(), Box<AstType>> {
                    let location = self.get_location();
                    self.next_lexeme(); // skip LeftSquareBracket
                    self.next_lexeme(); // skip value

                    if self.get_lexeme().ne(&LexType::RightSquareBracket) {
                        return Err(self.report_type_error(false, "table field"));
                    }
                    self.next_lexeme();

                    if self.get_lexeme().ne(&LexType::Semicolon) {
                        return Err(self.report_type_error(false, "table field"));
                    }
                    self.next_lexeme();

                    let type_ = self.parse_type_annotation()?;

                    if value.is_empty() {
                        return Err(self.report_type_error(false, "table field"));
                    } else {
                        props.push(TableProp::new(AstName::new(value), location, type_));
                    }

                    Ok(())
                };

                match self.get_ahead_lexeme() {
                    LexType::QuotedString(value) => {
                        parse_string_props_append(value)?;
                    }
                    LexType::RawString(value) => {
                        parse_string_props_append(value)?;
                    }
                    _ => {
                        if indexer.is_some() {
                            return Err(self.report_type_error(
                                false,
                                "cannot have more than one table indexer",
                            ));
                        } else {
                            indexer = Some(self.parse_type_indexer_annotation()?);
                        }
                    }
                }
            }
            if props.is_empty() && indexer.is_none() {
                let parse_indexer = || -> Result<(), Box<AstType>> {
                    let type_ = self.parse_type_annotation()?;

                    indexer = Some(TableIndexer::new(
                        TypeReference::new(
                            type_.get_location(),
                            None,
                            AstName::new(String::from("number")),
                            None,
                        ),
                        type_,
                        type_.get_location(),
                    ));
                    Ok(())
                };

                if let LexType::Name(_) = self.get_lexeme() {
                    if self.get_ahead_lexeme().eq(&LexType::Colon) {
                        parse_name_props_append()?;
                    } else {
                        parse_indexer()?;
                        break;
                    }
                } else {
                    parse_indexer()?;
                    break;
                }
            } else {
                parse_name_props_append()?;
            }

            match self.get_lexeme() {
                LexType::Comma | LexType::Semicolon => self.next_lexeme(),
                _ => break,
            }
        }

        let end = self.get_location().get_end();

        if self.get_lexeme().ne(&LexType::RightCurlyBracket) {
            return Err(self.report_type_error(false, "unexpected parse table type annotation"));
        }
        self.next_lexeme();

        Ok(TypeTable::new(LexLocation::new(begin, end), props, indexer))
    }

    fn parse_generic_type_list(
        &mut self,
        with_default_values: bool,
    ) -> Result<
        (
            Vec<(AstName, LexLocation, Option<Box<AstType>>)>,
            Vec<(AstName, LexLocation, Option<Box<AstType>>)>,
        ),
        Box<AstType>,
    > {
        let mut names: Vec<(AstName, LexLocation, Option<Box<AstType>>)> = Vec::new();
        let mut name_packs: Vec<(AstName, LexLocation, Option<Box<AstType>>)> = Vec::new();

        if self.get_lexeme().eq(&LexType::Less) {
            let begin = self.get_location().get_begin();
            self.next_lexeme();

            let mut seen_pack = false;
            let mut seen_default = false;

            loop {
                let (name, name_location) = self.parse_name()?;

                if self.get_lexeme() == LexType::Dot3 || seen_pack {
                    seen_pack = true;

                    if self.get_lexeme() == LexType::Dot3 {
                        self.next_lexeme();
                    }

                    if with_default_values && self.get_lexeme() == LexType::Assign {
                        seen_default = true;
                        self.next_lexeme();

                        if self.should_parse_type_pack_annotation() {
                            name_packs.push((
                                name,
                                name_location,
                                Some(self.parse_type_pack_annotation()?),
                            ));
                        } else if self.get_lexeme() == LexType::LeftRoundBracket {
                            name_packs.push((
                                name,
                                name_location,
                                Some(self.parse_type_or_pack_annotation()?),
                            ));
                        }
                    } else {
                        name_packs.push((name, name_location, None))
                    }
                } else {
                    if with_default_values && self.get_lexeme().eq(&LexType::Assign) {
                        seen_default = true;
                        self.next_lexeme();

                        names.push((name, name_location, Some(self.parse_type_annotation()?)));
                    } else {
                        names.push((name, name_location, None));
                    }
                }

                if self.get_lexeme() == LexType::Comma {
                    self.next_lexeme();
                } else {
                    break;
                }
            }

            if self.get_lexeme().ne(&LexType::Greater) {
                return Err(self.report_type_error(false, "unexpected generic type list"));
            }
            self.next_lexeme();
        }

        Ok((names, name_packs))
    }

    fn parse_simple_type_annotation(
        &mut self,
        allow_pack: bool,
    ) -> Result<(Option<Box<AstType>>, Option<Box<AstTypePack>>), Box<AstType>> {
        let begin = self.get_location().get_begin();

        Ok(match self.get_lexeme() {
            LexType::Nil => (Some(self.parse_nil_type()?), None),
            LexType::True | LexType::False => (Some(self.parse_bool_type()?), None),
            LexType::QuotedString(_) | LexType::RawString(_) | LexType::BrokenString => {
                (Some(self.parse_string_type()?), None)
            }
            LexType::Name(_) => (Some(self.parse_name_or_typeof_type()?), None),
            LexType::LeftCurlyBracket => (Some(self.parse_table_type_annotation()?), None),
            // TODO function type annotation
            _ => return Err(self.report_type_error(true, "unexpected type")),
        })
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

            if let Ok(result) = parser.parse_number_expr() {
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
