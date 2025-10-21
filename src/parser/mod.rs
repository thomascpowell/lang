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

    fn expect(&mut self, expected: &TokenKind) -> Result<Token, Error> {
        // should be able to expect a certain token type
        Err(Error::new(ErrorType::Default, 0, 0, "", Some("")))
    }
}
