use super::{LexLocation, LexPosition, LexType, Lexeme};
use std::str::Chars;

pub struct Lexer<'src_lf> {
    src_p: Chars<'src_lf>,
    current_char: Option<char>,

    offset: u32,
    line: u32,
    line_offset: u32,

    lexeme: Lexeme,

    previous_location: LexLocation,
}

impl<'src_lf> Lexer<'src_lf> {
    pub fn new(src: &'src_lf str) -> Self {
        let mut chars = src.chars();
        let current_char = chars.next();

        Lexer {
            src_p: chars,
            current_char,
            offset: 0,
            line: 0,
            line_offset: 0,
            lexeme: Lexeme::new(LexLocation::zero(), LexType::Eof),
            previous_location: LexLocation::zero(),
        }
    }

    fn is_space(ch: char) -> bool {
        match ch {
            ' ' | '\t' | '\r' | '\n' => true,
            _ => false,
        }
    }

    fn is_new_line(ch: char) -> bool {
        ch.eq(&'\n')
    }

    fn buf_to_string(buf: &Vec<char>) -> String {
        buf.iter().collect::<String>()
    }

    fn consume(&mut self) {
        if let Some(ch) = self.current_char {
            if Lexer::is_new_line(ch) {
                self.line += 1;
                self.line_offset = self.offset + 1;
            }

            self.offset += 1;
            self.current_char = self.src_p.next();
        }
    }

    fn position(&self) -> LexPosition {
        LexPosition::new(self.line, self.offset - self.line_offset)
    }

    fn is_comment(lexeme: &Lexeme) -> bool {
        match lexeme.get_type() {
            LexType::Comment(_) | LexType::BlockComment => true,
            _ => false,
        }
    }

    fn skip_space(&mut self) {
        loop {
            if let Some(ch) = self.current_char {
                if !Lexer::is_space(ch) {
                    break;
                }
            } else {
                break;
            }

            self.consume();
        }
    }

    fn skip_long_separator(&mut self, ch: char) -> i32 {
        let mut count = 0;

        loop {
            if let Some(ech) = self.current_char {
                if ech.eq(&'=') {
                    count += 1;
                    self.consume();

                    continue;
                }
            }

            break;
        }

        if self.current_char.eq(&Some(ch)) {
            self.consume();

            count
        } else {
            (-count) - 1
        }
    }

    fn read_long_string(
        &mut self,
        position: &LexPosition,
        sep: i32,
        wrap_fn: fn(&Vec<char>) -> LexType,
        broken: LexType,
    ) -> Lexeme {
        let mut buf: Vec<char> = Vec::new();

        loop {
            if let Some(ch) = self.current_char {
                self.consume();

                if ch.eq(&']') {
                    if self.skip_long_separator(ch).eq(&sep) {
                        return Lexeme::new(
                            LexLocation::new(*position, self.position()),
                            wrap_fn(&buf),
                        );
                    }
                } else {
                    buf.push(ch);
                }
            } else {
                return Lexeme::new(LexLocation::new(*position, self.position()), broken);
            }
        }
    }

    fn read_quoted_string(&mut self, delimiter: char) -> Lexeme {
        let start = self.position();

        let mut buf: Vec<char> = Vec::new();
        loop {
            if let Some(ch) = self.current_char {
                self.consume();

                if ch.eq(&delimiter) {
                    return Lexeme::new(
                        LexLocation::new(start, self.position()),
                        LexType::QuotedString(Lexer::buf_to_string(&buf)),
                    );
                } else {
                    match ch {
                        '\\' => {
                            if let Some(nch) = self.current_char {
                                self.consume();

                                match nch {
                                    '\r' | '\n' => {
                                        buf.push(nch);
                                    }
                                    'r' => buf.push('\n'),
                                    'n' => buf.push('\n'),
                                    't' => buf.push('\t'),
                                    '\\' => buf.push('\\'),
                                    _ => {
                                        return Lexeme::new(
                                            LexLocation::new(start, self.position()),
                                            LexType::BrokenString,
                                        )
                                    }
                                }
                            }
                        }
                        '\r' | '\n' => {
                            return Lexeme::new(
                                LexLocation::new(start, self.position()),
                                LexType::BrokenString,
                            )
                        }
                        _ => buf.push(ch),
                    }
                }
            } else {
                return Lexeme::new(
                    LexLocation::new(start, self.position()),
                    LexType::BrokenString,
                );
            }
        }
    }

    fn read_number(&mut self, ch: char, position: &LexPosition) -> Lexeme {
        let mut buf: Vec<char> = Vec::new();
        buf.push(ch);
        loop {
            if let Some(ch) = self.current_char {
                buf.push(ch);
            }
            self.consume();

            if let Some(ch) = self.current_char {
                if ch.is_digit(10) || ch.eq(&'.') || ch.eq(&'_') {
                    continue;
                }
            }
            break;
        }

        if let Some(ch) = self.current_char {
            if ch.eq_ignore_ascii_case(&'e') {
                buf.push(ch);
                self.consume();

                if let Some(ch) = self.current_char {
                    if ch.eq(&'+') || ch.eq(&'-') {
                        buf.push(ch);
                        self.consume();
                    }
                }
            }
        }

        while self.current_char.is_some()
            && (self.current_char.unwrap().is_alphabetic()
                || self.current_char.unwrap().is_digit(10)
                || self.current_char.eq(&Some('_')))
        {
            buf.push(self.current_char.unwrap());
            self.consume();
        }

        Lexeme::new(
            LexLocation::new(*position, self.position()),
            LexType::Number(Lexer::buf_to_string(&buf)),
        )
    }

    fn read_comment_body(&mut self) -> Lexeme {
        let start = self.position();

        if self.current_char.eq(&Some('[')) {
            let sep = self.skip_long_separator('[');

            if sep.ge(&0) {
                return self.read_long_string(
                    &start,
                    sep,
                    |x: &Vec<char>| LexType::Comment(Lexer::buf_to_string(x)),
                    LexType::BrokenComment,
                );
            }
        }

        let mut buf: Vec<char> = Vec::new();
        while self.current_char.is_some()
            && self.current_char.ne(&Some('\r'))
            && !Lexer::is_new_line(self.current_char.unwrap())
        {
            buf.push(self.current_char.unwrap());
            self.consume();
        }

        Lexeme::new(
            LexLocation::new(start, self.position()),
            LexType::Comment(Lexer::buf_to_string(&buf)),
        )
    }

    fn read_name(&mut self, ch: char, position: &LexPosition) -> Lexeme {
        let mut buf: Vec<char> = Vec::new();
        buf.push(ch);
        loop {
            if let Some(ch) = self.current_char {
                buf.push(ch);
            }

            self.consume();

            if self.current_char.is_some()
                && (self.current_char.unwrap().is_alphabetic()
                    || self.current_char.unwrap().is_digit(10)
                    || self.current_char.eq(&Some('_')))
            {
                continue;
            }
            break;
        }

        let name = Lexer::buf_to_string(&buf);
        Lexeme::new(
            LexLocation::new(*position, self.position()),
            match name.as_str() {
                "begin" => LexType::Begin,
                "and" => LexType::And,
                "break" => LexType::Break,
                "do" => LexType::Do,
                "else" => LexType::Else,
                "elseif" => LexType::ElseIf,
                "end" => LexType::End,
                "false" => LexType::False,
                "for" => LexType::For,
                "function" => LexType::Function,
                "if" => LexType::If,
                "in" => LexType::In,
                "local" => LexType::Local,
                "nil" => LexType::Nil,
                "not" => LexType::Not,
                "or" => LexType::Or,
                "repeat" => LexType::Repeat,
                "return" => LexType::Return,
                "then" => LexType::Then,
                "true" => LexType::True,
                "until" => LexType::Until,
                "while" => LexType::While,
                _ => LexType::Name(name),
            },
        )
    }

    fn read_next(&mut self) -> Lexeme {
        let start = self.position();

        if let Some(ch) = self.current_char {
            self.consume();

            match ch {
                '-' => match self.current_char {
                    Some('>') => {
                        self.consume();

                        Lexeme::new(LexLocation::line_offset(start, 2), LexType::SkinnyArrow)
                    }
                    Some('=') => {
                        self.consume();

                        Lexeme::new(LexLocation::line_offset(start, 2), LexType::SubAssign)
                    }
                    Some('-') => {
                        self.consume();

                        self.read_comment_body()
                    }
                    _ => Lexeme::new(LexLocation::line_offset(start, 1), LexType::Sub),
                },
                '+' => {
                    if self.current_char.eq(&Some('=')) {
                        self.consume();

                        Lexeme::new(LexLocation::line_offset(start, 2), LexType::AddAssign)
                    } else {
                        Lexeme::new(LexLocation::line_offset(start, 1), LexType::Add)
                    }
                }
                '*' => {
                    if self.current_char.eq(&Some('=')) {
                        self.consume();

                        Lexeme::new(LexLocation::line_offset(start, 2), LexType::MulAssign)
                    } else {
                        Lexeme::new(LexLocation::line_offset(start, 1), LexType::Mul)
                    }
                }
                '/' => {
                    if self.current_char.eq(&Some('=')) {
                        self.consume();

                        Lexeme::new(LexLocation::line_offset(start, 2), LexType::DivAssign)
                    } else {
                        Lexeme::new(LexLocation::line_offset(start, 1), LexType::Div)
                    }
                }
                '%' => {
                    if self.current_char.eq(&Some('=')) {
                        self.consume();

                        Lexeme::new(LexLocation::line_offset(start, 2), LexType::ModAssign)
                    } else {
                        Lexeme::new(LexLocation::line_offset(start, 1), LexType::Mod)
                    }
                }
                '^' => {
                    if self.current_char.eq(&Some('=')) {
                        self.consume();

                        Lexeme::new(LexLocation::line_offset(start, 2), LexType::PowAssign)
                    } else {
                        Lexeme::new(LexLocation::line_offset(start, 1), LexType::Pow)
                    }
                }
                '=' => {
                    if self.current_char.eq(&Some('=')) {
                        self.consume();

                        Lexeme::new(LexLocation::line_offset(start, 2), LexType::Equal)
                    } else {
                        Lexeme::new(LexLocation::line_offset(start, 1), LexType::Assign)
                    }
                }
                '<' => {
                    if self.current_char.eq(&Some('=')) {
                        self.consume();

                        Lexeme::new(LexLocation::line_offset(start, 2), LexType::LessEqual)
                    } else {
                        Lexeme::new(LexLocation::line_offset(start, 1), LexType::Less)
                    }
                }
                '>' => {
                    if self.current_char.eq(&Some('=')) {
                        self.consume();

                        Lexeme::new(LexLocation::line_offset(start, 2), LexType::GreaterEqual)
                    } else {
                        Lexeme::new(LexLocation::line_offset(start, 1), LexType::Greater)
                    }
                }
                '~' => {
                    if self.current_char.eq(&Some('=')) {
                        self.consume();

                        Lexeme::new(LexLocation::line_offset(start, 2), LexType::NotEqual)
                    } else {
                        Lexeme::new(LexLocation::line_offset(start, 1), LexType::Not)
                    }
                }
                ':' => {
                    if self.current_char.eq(&Some(':')) {
                        self.consume();

                        Lexeme::new(LexLocation::line_offset(start, 2), LexType::DoubleColon)
                    } else {
                        Lexeme::new(LexLocation::line_offset(start, 1), LexType::Colon)
                    }
                }
                '[' => {
                    let sep = self.skip_long_separator(ch);
                    if sep.ge(&0) {
                        self.read_long_string(
                            &start,
                            sep,
                            |x: &Vec<char>| LexType::RawString(Lexer::buf_to_string(x)),
                            LexType::BrokenString,
                        )
                    } else if sep.eq(&-1) {
                        Lexeme::new(
                            LexLocation::line_offset(start, 1),
                            LexType::LeftSquareBracket,
                        )
                    } else {
                        Lexeme::new(LexLocation::line_offset(start, 1), LexType::BrokenString)
                    }
                }
                '\'' | '\"' => self.read_quoted_string(ch),
                '.' => {
                    if self.current_char.eq(&Some('.')) {
                        self.consume();

                        if self.current_char.eq(&Some('.')) {
                            self.consume();

                            Lexeme::new(LexLocation::line_offset(start, 3), LexType::Dot3)
                        } else if self.current_char.eq(&Some('=')) {
                            self.consume();

                            Lexeme::new(LexLocation::line_offset(start, 3), LexType::ConcatAssign)
                        } else {
                            Lexeme::new(LexLocation::line_offset(start, 2), LexType::Dot2)
                        }
                    } else {
                        if let Some(nch) = self.current_char {
                            if nch.is_digit(10) {
                                self.read_number(ch, &start)
                            } else {
                                Lexeme::new(LexLocation::line_offset(start, 1), LexType::Dot)
                            }
                        } else {
                            Lexeme::new(LexLocation::line_offset(start, 1), LexType::Dot)
                        }
                    }
                }
                '(' => Lexeme::new(
                    LexLocation::line_offset(start, 1),
                    LexType::LeftRoundBracket,
                ),
                ')' => Lexeme::new(
                    LexLocation::line_offset(start, 1),
                    LexType::RightRoundBracket,
                ),
                '{' => Lexeme::new(
                    LexLocation::line_offset(start, 1),
                    LexType::LeftCurlyBracket,
                ),
                '}' => Lexeme::new(
                    LexLocation::line_offset(start, 1),
                    LexType::RightCurlyBracket,
                ),
                ']' => Lexeme::new(
                    LexLocation::line_offset(start, 1),
                    LexType::RightSquareBracket,
                ),
                ';' => Lexeme::new(LexLocation::line_offset(start, 1), LexType::Semicolon),
                ',' => Lexeme::new(LexLocation::line_offset(start, 1), LexType::Comma),
                '#' => Lexeme::new(LexLocation::line_offset(start, 1), LexType::Sharp),
                _ => {
                    if ch.is_digit(10) {
                        self.read_number(ch, &start)
                    } else if ch.is_alphabetic() || ch.eq(&'_') {
                        self.read_name(ch, &start)
                    } else {
                        Lexeme::new(LexLocation::line_zero(start), LexType::Eof)
                    }
                }
            }
        } else {
            Lexeme::new(LexLocation::line_zero(start), LexType::Eof)
        }
    }

    pub fn next(&mut self, skip_comments: bool) -> Lexeme {
        loop {
            self.skip_space();
            self.previous_location = self.lexeme.get_location();

            self.lexeme = self.read_next();

            if skip_comments && Lexer::is_comment(&self.lexeme) {
                continue;
            }
            break;
        }

        self.lexeme.clone()
    }

    pub fn get_previous_location(&self) -> LexLocation {
        self.previous_location
    }

    pub fn get_current(&self) -> Lexeme {
        self.lexeme.clone()
    }

    pub const fn get_current_ref(&self) -> &Lexeme {
        &self.lexeme
    }

    pub const fn get_current_location(&self) -> LexLocation {
        self.get_current_ref().get_location()
    }

    pub fn get_current_type(&self) -> LexType {
        self.get_current_ref().get_type()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_single(src: &str, type_: LexType) {
        let mut lexer = Lexer::new(src);
        let lexeme = lexer.next(false);

        assert_eq!(lexeme.get_type(), type_);
    }

    #[test]
    fn test_lexeme_single() {
        let assert_list = vec![
            ("begin", LexType::Begin),
            ("and", LexType::And),
            ("break", LexType::Break),
            ("do", LexType::Do),
            ("else", LexType::Else),
            ("elseif", LexType::ElseIf),
            ("end", LexType::End),
            ("false", LexType::False),
            ("for", LexType::For),
            ("function ", LexType::Function),
            (" if", LexType::If),
            ("in", LexType::In),
            ("local", LexType::Local),
            ("nil", LexType::Nil),
            ("not", LexType::Not),
            ("or", LexType::Or),
            ("repeat", LexType::Repeat),
            ("return", LexType::Return),
            ("then", LexType::Then),
            ("true", LexType::True),
            ("until", LexType::Until),
            ("while", LexType::While),
            ("_var1", LexType::Name(String::from("_var1"))),
            (" _var1", LexType::Name(String::from("_var1"))),
            ("_var1 ", LexType::Name(String::from("_var1"))),
            ("_var1 _var2", LexType::Name(String::from("_var1"))),
            ("123", LexType::Number(String::from("123"))),
            ("123.456", LexType::Number(String::from("123.456"))),
            (".456", LexType::Number(String::from(".456"))),
            ("-", LexType::Sub),
            ("-a", LexType::Sub),
            ("-1", LexType::Sub),
            ("->", LexType::SkinnyArrow),
            ("-=", LexType::SubAssign),
            (
                "--comment body",
                LexType::Comment(String::from("comment body")),
            ),
            ("+", LexType::Add),
            ("+a", LexType::Add),
            ("+a", LexType::Add),
            ("+.0", LexType::Add),
            ("+=", LexType::AddAssign),
            ("*", LexType::Mul),
            ("*=", LexType::MulAssign),
            ("/", LexType::Div),
            ("/=", LexType::DivAssign),
            ("%", LexType::Mod),
            ("%=", LexType::ModAssign),
            ("^", LexType::Pow),
            ("^=", LexType::PowAssign),
            ("=", LexType::Assign),
            ("==", LexType::Equal),
            ("<", LexType::Less),
            ("<=", LexType::LessEqual),
            (">", LexType::Greater),
            (">=", LexType::GreaterEqual),
            ("~", LexType::Not),
            ("~=", LexType::NotEqual),
            (":", LexType::Colon),
            ("::", LexType::DoubleColon),
            ("[", LexType::LeftSquareBracket),
            (
                "[===[hello world]===]",
                LexType::RawString(String::from("hello world")),
            ),
            ("[===[hello world", LexType::BrokenString),
            ("[===[hello world]====]", LexType::BrokenString),
            ("'foobar'", LexType::QuotedString(String::from("foobar"))),
            ("\"foobar\"", LexType::QuotedString(String::from("foobar"))),
            (
                "\"foo\\\nbar\"",
                LexType::QuotedString(String::from("foo\nbar")),
            ),
            (".a", LexType::Dot),
            (".", LexType::Dot),
            ("..", LexType::Dot2),
            ("...", LexType::Dot3),
            ("..=", LexType::ConcatAssign),
            ("(", LexType::LeftRoundBracket),
            (")", LexType::RightRoundBracket),
            ("{", LexType::LeftCurlyBracket),
            ("}", LexType::RightCurlyBracket),
            (";", LexType::Semicolon),
            (",", LexType::Comma),
            ("#", LexType::Sharp),
        ];

        for item in assert_list {
            let (keyword, lex_type) = item;
            assert_single(keyword, lex_type);
        }
    }

    #[test]
    fn test_sample_program() {
        let src = "
        function hello_world(statement)
            print(statement)
        end

        -- print \"foo bar\"
        hello_world('foo bar')

        ";
        let expect_vec = vec![
            LexType::Function,
            LexType::Name(String::from("hello_world")),
            LexType::LeftRoundBracket,
            LexType::Name(String::from("statement")),
            LexType::RightRoundBracket,
            LexType::Name(String::from("print")),
            LexType::LeftRoundBracket,
            LexType::Name(String::from("statement")),
            LexType::RightRoundBracket,
            LexType::End,
            LexType::Comment(String::from(" print \"foo bar\"")),
            LexType::Name(String::from("hello_world")),
            LexType::LeftRoundBracket,
            LexType::QuotedString(String::from("foo bar")),
            LexType::RightRoundBracket,
        ];

        let mut lexer = Lexer::new(src);

        let mut actual_vec: Vec<LexType> = Vec::new();
        loop {
            let lexeme = lexer.next(false);

            if lexeme.get_type().eq(&LexType::Eof) {
                break;
            } else {
                actual_vec.push(lexeme.get_type());
            }
        }

        assert_eq!(expect_vec, actual_vec);
    }
}
