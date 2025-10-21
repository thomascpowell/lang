use crate::{error_types::*, lexer::token::*};

pub mod ast;

/*
* Parser
* */

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens: tokens,
            pos: 0,
        }
    }

    fn parse_statement_list(&mut self) {
        // start the top down parse
    }

    fn has_next(&mut self) -> bool {
        self.peek().is_some()
    }

    fn peek_n(&self, n: usize) -> Option<Token> {
        self.tokens.get(self.pos + n).cloned()
    }

    fn peek(&self) -> Option<Token> {
        self.peek_n(0)
    }

    fn advance_n(&mut self, n: usize) -> Option<Token> {
        let tok = self.peek();
        if tok.is_some() {
            self.pos += n;
        }
        tok
    }

    fn advance(&mut self) -> Option<Token> {
        self.advance_n(1)
    }

    // matches a provided condition, returns the token or an error
    fn expect<F>(&mut self, cond: F) -> Result<Token, Error>
    where
        F: Fn(&TokenKind) -> bool,
    {
        match self.advance() {
            Some(tok) if cond(&tok.kind) => Ok(tok),
            Some(tok) => Err(Error::new(
                ErrorType::UnexpectedTokenType,
                tok.line,
                tok.col,
                format!("{:?}", tok.kind),
                None,
            )),
            None => Err(Error::new(ErrorType::UnexpectedEOF, 0, 0, "EOF", None)),
        }
    }
}
