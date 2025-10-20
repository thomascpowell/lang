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

    fn peek(&self) -> Option<Token> {
        self.tokens.get(self.pos).cloned()
    }

    fn has_next(&mut self) -> bool {
        self.peek().is_some()
    }

    fn advance(&mut self) -> Option<Token> {
        let tok = self.peek();
        if tok.is_some() {
            self.pos += 1;
        }
        tok
    }

    fn expect(&mut self, expected: &TokenKind) -> Result<Token, Error> {
        // should be able to expect a certain token type
        Err(Error::new(ErrorType::Default, 0, 0, "", Some("")))
    }
}
