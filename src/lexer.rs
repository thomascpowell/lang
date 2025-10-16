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
            src: get_chars(&input),
            pos: 0,
            line: 1,
            col: 1,
        }
    }
    pub fn next_token(&mut self) -> Result<Token, LexerError> {
        Err(LexerError::Default)
    }

    pub fn has_next(&mut self) -> bool {
        self.peek().is_some()
    }

    fn peek(&self) -> Option<char> {
        self.src.get(self.pos).cloned()
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
            Some(c)
        } else {
            None
        }
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

pub fn get_chars(source: &str) -> Vec<char> {
    source.chars().collect()
}
