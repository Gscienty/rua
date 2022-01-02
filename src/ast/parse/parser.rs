use super::super::*;

pub struct ParserOptions {
    allow_type_annotations: bool,
    support_continue_statement: bool,
    allow_declaration_syntax: bool,
    capture_comments: bool,
}

pub struct Parser<'src_lf> {
    lex: Lexer<'src_lf>,

    name_self: AstName,
    name_number: AstName,
    name_error: AstName,
    name_nil: AstName,

    token_step: Vec<i32>,
    options: ParserOptions,
    comments: Vec<Comment>,
}

impl<'src_lf> Parser<'src_lf> {
    pub fn new(src: &'src_lf str, names: &mut NameTable, options: ParserOptions) -> Self {
        let name_self = names.add("self", LexType::Name(String::from("self")));
        let name_number = names.add("number", LexType::Name(String::from("number")));
        let name_error = names.add("%error-id%", LexType::Name(String::from("%error-id%")));
        let name_nil = names.add("nil", LexType::Name(String::from("nil")));

        let mut parser = Parser {
            lex: Lexer::new(src),
            name_self,
            name_number,
            name_error,
            name_nil,

            token_step: Vec::new(),
            options,
            comments: Vec::new(),
        };
        parser.token_step.resize(LexType::count() as usize, 0);
        *parser
            .token_step
            .get_mut(LexType::Eof.code() as usize)
            .unwrap() = 1;

        parser.next_lexeme();

        parser
    }

    fn next_lexeme(&mut self) -> Lexeme {
        if self.options.capture_comments {
            loop {
                let lexeme = self.lex.next(false);

                match lexeme.get_type() {
                    LexType::Comment(_) | LexType::BlockComment | LexType::BrokenComment => self
                        .comments
                        .push(Comment::new(lexeme.get_type(), lexeme.get_location())),
                    _ => return lexeme,
                }
            }
        } else {
            self.lex.next(false)
        }
    }
}
