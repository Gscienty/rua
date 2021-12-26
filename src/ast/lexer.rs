use super::{LexLocation, LexPosition, LexType, Lexeme};
use std::str::Chars;

pub struct Lexer<'src_lf> {
    src: &'src_lf str,
    src_p: Chars<'src_lf>,
    current_char: Option<char>,

    offset: u32,
    line: u32,
    line_offset: u32,

    lexeme: Lexeme,

    prev_location: LexLocation,

    skip_comments: bool,
    read_names: bool,
}

impl<'src_lf> Lexer<'src_lf> {
    pub fn new(src: &'src_lf str) -> Self {
        let mut chars = src.chars();
        let current_char = chars.next();

        Lexer {
            src,
            src_p: chars,
            current_char,
            offset: 0,
            line: 0,
            line_offset: 0,
            lexeme: Lexeme::new(LexLocation::zero(), LexType::Eof),
            prev_location: LexLocation::zero(),
            skip_comments: false,
            read_names: true,
        }
    }

    pub fn set_skip_comments(&mut self, skip: bool) {
        self.skip_comments = skip;
    }

    pub fn set_read_names(&mut self, read: bool) {
        self.read_names = read;
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

    fn skip_long_separator(&mut self) -> i32 {
        self.consume();

        let mut count = 0;
        let start = self.current_char;

        while self.current_char.eq(&Some('=')) {
            self.consume();
            count += 1;
        }

        if self.current_char.eq(&start) {
            count
        } else {
            (-count) - 1
        }
    }

    fn read_long_string(
        &mut self,
        position: &LexPosition,
        sep: i32,
        wrap_fn: fn(Vec<char>) -> LexType,
        broken: LexType,
    ) -> Lexeme {
        self.consume();

        let mut buf: Vec<char> = Vec::new();
        while self.current_char.is_some() {
            if self.current_char.eq(&Some(']')) {
                if self.skip_long_separator().eq(&sep) {
                    self.consume();

                    return Lexeme::new(LexLocation::new(*position, self.position()), wrap_fn(buf));
                }
            } else {
                if let Some(ch) = self.current_char {
                    buf.push(ch);
                }

                self.consume();
            }
        }

        Lexeme::new(LexLocation::new(*position, self.position()), broken)
    }

    fn read_quoted_string(&mut self) -> Lexeme {
        let start = self.position();

        let delimiter = self.current_char;
        self.consume();

        let mut buf: Vec<char> = Vec::new();
        while self.current_char.ne(&delimiter) {
            match self.current_char {
                None | Some('\r') | Some('\n') => {
                    return Lexeme::new(
                        LexLocation::new(start, self.position()),
                        LexType::BrokenString,
                    );
                }
                // TODO \r \n \t etc.
                _ => {
                    if let Some(ch) = self.current_char {
                        buf.push(ch);
                    }
                }
            }

            self.consume();
        }

        Lexeme::new(
            LexLocation::new(start, self.position()),
            LexType::QuotedString(buf),
        )
    }

    fn read_number(&mut self, position: &LexPosition) -> Lexeme {
        let mut buf: Vec<char> = Vec::new();
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
            LexType::Number(buf),
        )
    }

    fn read_comment_body(&mut self) -> Lexeme {
        let start = self.position();

        if self.current_char.eq(&Some('[')) {
            let sep = self.skip_long_separator();

            if sep.ge(&0) {
                return self.read_long_string(
                    &start,
                    sep,
                    |x: Vec<char>| LexType::Comment(x),
                    LexType::BrokenComment,
                );
            }
        }

        let buf: Vec<char> = Vec::new();
        while self.current_char.is_some()
            && self.current_char.ne(&Some('\r'))
            && !Lexer::is_new_line(self.current_char.unwrap())
        {
            buf.push(self.current_char.unwrap());
            self.consume();
        }

        Lexeme::new(
            LexLocation::new(start, self.position()),
            LexType::Comment(buf),
        )
    }

    fn read_name(&mut self, position: &LexPosition) -> Lexeme {
        let mut buf: Vec<char> = Vec::new();
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

        let name = buf.iter().collect::<String>();
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
                _ => LexType::Name(buf),
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
                    let sep = self.skip_long_separator();
                    if sep.ge(&0) {
                        self.read_long_string(
                            &start,
                            sep,
                            |x: Vec<char>| LexType::RawString(x),
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
                '\'' | '\"' => self.read_quoted_string(),
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
                        if let Some(ch) = self.current_char {
                            if ch.is_digit(10) {
                                self.read_number(&start)
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
                    if let Some(ch) = self.current_char {
                        if ch.is_digit(10) {
                            self.read_number(&start)
                        } else if ch.is_alphabetic() || ch.eq(&'_') {
                            self.read_number(&start)
                        } else {
                            Lexeme::new(LexLocation::line_zero(start), LexType::Eof)
                        }
                    } else {
                        Lexeme::new(LexLocation::line_zero(start), LexType::Eof)
                    }
                }
            }
        } else {
            Lexeme::new(LexLocation::line_zero(start), LexType::Eof)
        }
    }

    fn next(&mut self, skip_comments: bool) -> Lexeme {
        loop {
            self.skip_space();
            self.prev_location = self.lexeme.get_location();

            self.lexeme = self.read_next();

            if skip_comments && Lexer::is_comment(&self.lexeme) {
                continue;
            }
            break;
        }

        self.lexeme
    }
}
