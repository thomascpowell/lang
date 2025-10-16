use crate::error_types::*;
use crate::token_types::*;

struct Lexer {
    src: Vec<char>,
    pos: usize,
    line: usize,
    col: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self {
            src: input.chars().collect(),
            pos: 0,
            line: 1,
            col: 1,
        }
    }
    pub fn next_token(&mut self) -> Result<Token, LexerError> {
        self.skip_whitespace();

        let start_line = self.line;
        let start_col = self.col;

        let c = self.peek().ok_or(LexerError::UnexpectedEOF)?;

        let token = match c {
            c if c.is_alphanumeric() => {
                let identifier = self.consume_while(|c| c.is_alphanumeric());
                classify_keyword_or_identifier(identifier)
            }


            // temporary default case to silence static analysis
            _ => TokenKind::Keyword(Keyword::False)
        };

        Err(LexerError::Default)
    }

    pub fn has_next(&mut self) -> bool {
        self.peek().is_some()
    }

    fn consume_while<F>(&mut self, mut predicate: F) -> String
    where
        F: FnMut(char) -> bool,
    {
        let mut buf = String::new();
        while let Some(c) = self.peek() {
            if predicate(c) {
                buf.push(c);
                self.advance();
            } else {
                break;
            }
        }
        buf
    }

    fn peek(&self) -> Option<char> {
        self.src.get(self.pos).cloned()
    }

    fn skip_whitespace(&mut self) {
        while self.peek().unwrap().is_whitespace() {
            self.advance();
        }
    }

    fn advance(&mut self) -> Option<char> {
        if let Some(c) = self.peek() {
            self.pos += 1;
            if c == '\n' {
                self.line += 1;
                self.col = 1;
            } else {
                self.col += 1;
            }
            return Some(c);
        }
        None
    }
}

pub fn tokenize(input: String) -> Result<Vec<Token>, LexerError> {
    let mut res = Vec::new();
    let mut lexer = Lexer::new(input);
    while lexer.has_next() {
        res.push(lexer.next_token()?)
    }
    Ok(res)
}

fn classify_keyword_or_identifier(identifier: String) -> TokenKind {
    match identifier.as_str() {
        "fn" => TokenKind::Keyword(Keyword::Fn),
        "i32" => TokenKind::Keyword(Keyword::I32),
        "u32" => TokenKind::Keyword(Keyword::U32),
        "bool" => TokenKind::Keyword(Keyword::Bool),
        "string" => TokenKind::Keyword(Keyword::String),
        "true" => TokenKind::Keyword(Keyword::True),
        "false" => TokenKind::Keyword(Keyword::False),
         _ => TokenKind::Identifier(identifier),
    }
}
