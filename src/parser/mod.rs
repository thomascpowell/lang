use crate::{error_types::*, lexer::token::*, parser::ast::*};

pub mod ast;

/*
* Parser
* */

pub fn parse(tokens: Vec<Token>) -> Result<StatementList, Error> {
    let mut res = Vec::new();
    let mut parser = Parser::new(tokens);
    while parser.has_next() {
        let statement = parser.parse_statement()?;
        res.push(statement);
    }
    Ok(StatementList { statements: res })
}

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

    // top down parser
    // design:
    // - caller validates tokens (e.g. return statement begins w/ return)
    // - callee consumes the token
    // - callee returns the lowest level possible (e.g. parse_return -> Ok(Return))

    fn parse_statement(&mut self) -> Result<Statement, Error> {
        let tok = self
            .peek()
            .ok_or_else(|| Error::generic_eof("expected a statement"))?;
        match &tok.kind {
            // statement: return
            TokenKind::Keyword(Keyword::Return) => Ok(Statement::Return(self.parse_return()?)),
            // match assignments (only other valid use of keywords)
            kind if is_type(kind) => Ok(Statement::Assignment(self.parse_assignment()?)),
            // other keyword (not a valid statement)
            TokenKind::Keyword(k) => {
                let found = format!("{:?}", k);
                Err(Error::new(
                    ErrorType::UnexpectedTokenType,
                    tok.line,
                    tok.col,
                    found,
                    Some("unexpected keyword"),
                ))
            }
            // everything else is an expression
            // may or may not be valid though
            _ => Ok(Statement::Expression(self.parse_expression()?)),
        }
    }

    fn parse_return(&mut self) -> Result<Return, Error> {
        // get the return token
        // or generic error because this should be impossible (caller validated)
        let tok = self.advance().ok_or_else(|| Error::generic())?;
        // store the start position
        let pos = Position {
            start_line: tok.line,
            start_col: tok.col,
        };
        // parse the expression that follows
        let expr = self.parse_expression()?;
        let ret = Return {
            position: pos,
            expression: expr,
        };
        Ok(ret)
    }

    fn parse_assignment(&mut self) -> Result<Assignment, Error> {
        todo!();
    }

    fn parse_expression(&mut self) -> Result<Expression, Error> {
        todo!();
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
    // accepts a closure containing a match macro
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
            None => Err(Error::generic_eof("unknown")),
        }
    }

    // like expect, but used for conditionals
    // probably wont need this long term
    fn compare_kind<F>(&self, cond: F) -> bool
    where
        F: Fn(&TokenKind) -> bool,
    {
        match self.peek() {
            Some(tok) if cond(&tok.kind) => true,
            _ => false,
        }
    }
}

// returns true if token kind is a type
fn is_type(kind: &TokenKind) -> bool {
    match kind {
        TokenKind::Keyword(Keyword::I32) => true,
        TokenKind::Keyword(Keyword::String) => true,
        TokenKind::Keyword(Keyword::Bool) => true,
        _ => false,
    }
}
