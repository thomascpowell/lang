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
        // need to report the line the token starts on
        // this stuff will useful for error reporting later on probably
        let start_line = self.line;
        let start_col = self.col;
        // get the start character
        let c = self.peek().ok_or(LexerError::new(
            LexerErrorType::UnexpectedEOF,
            start_line,
            start_col,
            "EOF",
            None,
        ))?;
        // match every type of character
        let (kind, original) = match c {
            // Literal (Int)
            c if c.is_ascii_digit() => {
                let digits = self.consume_while(|c| c.is_ascii_digit());
                let val = digits
                    .parse()
                    .map_err(|_| {
                        LexerError::new(
                            LexerErrorType::InvalidIntLiteral,
                            start_line,
                            start_col,
                            digits,
                            Some("probably overflow (i32)"),
                        )
                    })?;
                (TokenKind::Literal(Literal::Int(val)), digits)
            }
            // Identifier, Keyword
            c if c.is_alphanumeric() => {
                let t = self.consume_while(|c| c.is_alphanumeric());
                (classify_keyword_or_identifier(&t), t)
            }
            // Literal (String)
            '"' => {
                self.advance();
                let s = self.consume_while(|c| c != '"' && c != '\0');
                if !self.has_next() {
                    return Err(LexerError::UnterminatedStringLiteral(
                        "TEMPORARY ERROR MESSAGE".to_string(),
                    ));
                }
                self.advance();
                (TokenKind::Literal(Literal::String(s.clone())), s)
            }
            // Comment
            c if c == '/' && self.peek_next().is_some_and(|c| c == '/') => {
                self.advance_n(2);
                let comment = self.consume_while(|c| c != '\n');
                (TokenKind::Comment(comment.clone()), comment)
            }
            // Separators
            '(' => self.make_simple_token(TokenKind::Separator(Separator::LParen), '('),
            ')' => self.make_simple_token(TokenKind::Separator(Separator::RParen), ')'),
            '{' => self.make_simple_token(TokenKind::Separator(Separator::LBrace), '{'),
            '}' => self.make_simple_token(TokenKind::Separator(Separator::RBrace), '}'),
            ',' => self.make_simple_token(TokenKind::Separator(Separator::Comma), ','),
            ';' => self.make_simple_token(TokenKind::Separator(Separator::Semicolon), ';'),

            // Operators (Double)
            '!' if self.peek_next().is_some_and(|c| c == '=') => {
                self.advance_n(2);
                (TokenKind::Operator(Operator::Ne), "!=".to_string())
            }
            '<' if self.peek_next().is_some_and(|c| c == '=') => {
                self.advance_n(2);
                (TokenKind::Operator(Operator::Le), "<=".to_string())
            }
            '>' if self.peek_next().is_some_and(|c| c == '=') => {
                self.advance_n(2);
                (TokenKind::Operator(Operator::Ge), ">=".to_string())
            }
            '&' if self.peek_next().is_some_and(|c| c == '&') => {
                self.advance_n(2);
                (TokenKind::Operator(Operator::And), "&&".to_string())
            }
            '|' if self.peek_next().is_some_and(|c| c == '|') => {
                self.advance_n(2);
                (TokenKind::Operator(Operator::Or), "||".to_string())
            }
            // Operators (Single)
            '+' => self.make_simple_token(TokenKind::Operator(Operator::Add), '+'),
            '-' => self.make_simple_token(TokenKind::Operator(Operator::Sub), '-'),
            '*' => self.make_simple_token(TokenKind::Operator(Operator::Mul), '*'),
            '/' => self.make_simple_token(TokenKind::Operator(Operator::Div), '/'),
            '<' => self.make_simple_token(TokenKind::Operator(Operator::Lt), '<'),
            '>' => self.make_simple_token(TokenKind::Operator(Operator::Gt), '>'),
            '!' => self.make_simple_token(TokenKind::Operator(Operator::Not), '!'),
            '=' => self.make_simple_token(TokenKind::Operator(Operator::Assign), '='),

            // TODO: Create function that takes contextual info and turns it into a good error
            _ => {
                return Err(LexerError::InvalidChar(
                    "TEMPORARY ERROR MESSAGE".to_string(),
                ));
            }
        };
        let token = Token {
            kind,
            original,
            line: start_line,
            col: start_col,
        };
        Ok(token)
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

    fn peek_next(&self) -> Option<char> {
        self.src.get(self.pos + 1).cloned()
    }

    fn skip_whitespace(&mut self) {
        while self.peek().is_some_and(|c| c.is_whitespace()) {
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

    fn advance_n(&mut self, n: usize) {
        for _ in 0..n {
            self.advance();
        }
    }

    fn make_simple_token(&mut self, kind: TokenKind, ch: char) -> (TokenKind, String) {
        self.advance();
        (kind, ch.to_string())
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

fn classify_keyword_or_identifier(identifier: &str) -> TokenKind {
    match identifier {
        "fn" => TokenKind::Keyword(Keyword::Fn),
        "i32" => TokenKind::Keyword(Keyword::I32),
        "bool" => TokenKind::Keyword(Keyword::Bool),
        "string" => TokenKind::Keyword(Keyword::String),
        "true" => TokenKind::Keyword(Keyword::True),
        "false" => TokenKind::Keyword(Keyword::False),
        _ => TokenKind::Identifier(identifier.to_string()),
    }
}
