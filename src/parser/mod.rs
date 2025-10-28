use crate::{error_types::*, lexer::token::*, parser::ast::*};
pub mod ast;

/*
* Parser
* */

/*
* design notes:
* - caller validates tokens (e.g. return statement begins w/ return)
* - callee consumes the token (self.advance())
* - callee returns the lowest level possible (e.g. parse_return -> Ok(Return))
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
            _ => Ok(Statement::Expression(self.parse_expression(0)?)),
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
        let expr = self.parse_expression(0)?;
        let ret = Return {
            position: pos,
            expression: expr,
        };
        Ok(ret)
    }

    fn parse_assignment(&mut self) -> Result<Assignment, Error> {
        let type_tok = self.expect(|x| matches!(x, TokenKind::Keyword(_)))?;
        let pos = Position {
            start_line: type_tok.line,
            start_col: type_tok.col,
        };
        match type_tok.kind {
            TokenKind::Keyword(Keyword::I32) => self.handle_assignment(Type::I32, pos),
            TokenKind::Keyword(Keyword::Bool) => self.handle_assignment(Type::Bool, pos),
            TokenKind::Keyword(Keyword::String) => self.handle_assignment(Type::String, pos),
            _ => Err(Error::new(
                ErrorType::UnexpectedTokenType,
                type_tok.line,
                type_tok.col,
                type_tok.original,
                Some("expected: i32, bool, or string"),
            )),
        }
    }

    fn handle_assignment(&mut self, a_type: Type, pos: Position) -> Result<Assignment, Error> {
        let identifier = self.expect(|k| matches!(k, TokenKind::Identifier(_)))?;
        let ident_str = match identifier.kind {
            TokenKind::Identifier(s) => s,
            _ => unreachable!(),
        };
        Ok(Assignment {
            position: pos,
            assignment_type: a_type,
            identifier: ident_str,
            expression: self.parse_expression(0)?,
        })
    }

    fn parse_expression(&mut self, min_prec: u8) -> Result<Expression, Error> {
        let tok = self
            .peek()
            .ok_or_else(|| Error::generic_eof("expected a statement"))?;
        let pos = Position {
            start_line:tok.line,
            start_col: tok.col
        };
        // prefix expressions
        match &tok.kind {
            TokenKind::Keyword(Keyword::Fn) => {
                return Ok(Expression::Function(self.parse_function()?));
            }
            TokenKind::Keyword(Keyword::If) => return Ok(Expression::IfExp(self.parse_if_expr()?)),
            TokenKind::Separator(Separator::LParen) => return self.parse_paren_expr(),
            _ => {}
        }
        // pratt parsing
        let mut lhs = match tok.kind {
            TokenKind::Literal(_) => self.parse_literal()?,
            TokenKind::Identifier(_) => self.parse_identifier()?,
            _ => {
                return Err(Error {
                    error_type: ErrorType::UnexpectedTokenType,
                    start_line: tok.line,
                    start_col: tok.line,
                    found: tok.original,
                    message: Some("unexpected token".to_string()),
                });
            }
        };
        while let Some(tok) = self.peek() {
            let op = match &tok.kind {
                TokenKind::Operator(op) => op.clone(),
                _ => break,
            };
            let prec = op.get_precedence();
            if prec < min_prec {
                break;
            }
            self.advance();
            let rhs = self.parse_expression(prec + 1)?;
            lhs = Expression::BinaryExp(BinaryExp {
                position: pos.clone(),
                left: Box::new(lhs),
                right: Box::new(rhs),
                operator: op,
            });
        }
        Ok(lhs)
    }

    fn parse_literal(&mut self) -> Result<Expression, Error> {
        todo!()
    }
    fn parse_identifier(&mut self) -> Result<Expression, Error> {
        todo!()
    }
    fn parse_function(&mut self) -> Result<Function, Error> {
        todo!()
    }
    fn parse_if_expr(&mut self) -> Result<IfExp, Error> {
        todo!()
    }
    fn parse_paren_expr(&mut self) -> Result<Expression, Error> {
        todo!()
    }

    /*
     * utility functions
     * */

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
