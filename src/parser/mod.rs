use crate::parser::ast::Literal;
use crate::parser::ast::Operator;
use crate::position::Position;
use crate::{lang_error::*, lexer::token::*, parser::ast::*};
pub mod ast;

/*
* Parser
* */

pub fn parse(tokens: Vec<Token>) -> Result<StatementList, Error> {
    let mut res = Vec::new();
    let mut parser = Parser::new(tokens);
    while parser.has_next() {
        if !parser.has_next() {
            break;
        }
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
        let res = match &tok.kind {
            // statement: return
            TokenKind::Keyword(Keyword::Return) => Ok(Statement::Return(self.parse_return()?)),
            // match assignments (only other valid use of keywords)
            kind if is_type(kind) => Ok(Statement::Assignment(self.parse_assignment()?)),
            // everything else is an expression
            // may or may not be valid though
            _ => Ok(Statement::Expression(self.parse_expression(0)?)),
        };
        self.optional(|x| matches!(x, TokenKind::Separator(Separator::Semicolon)));
        res
    }

    fn parse_return(&mut self) -> Result<Return, Error> {
        // not needed, the expression already conveys that info
        // let tok = self.advance().ok_or_else(|| Error::generic())?;
        self.advance();
        // parse the expression
        let ret = Return {
            expression: self.parse_expression(0)?,
        };
        Ok(ret)
    }

    fn parse_assignment(&mut self) -> Result<Assignment, Error> {
        let type_tok = self.expect(|x| matches!(x, TokenKind::Keyword(_)))?;
        let pos = type_tok.position.clone();
        match type_tok.kind {
            TokenKind::Keyword(Keyword::I32) => self.handle_assignment(Type::I32, pos),
            TokenKind::Keyword(Keyword::F32) => self.handle_assignment(Type::F32, pos),
            TokenKind::Keyword(Keyword::Bool) => self.handle_assignment(Type::Bool, pos),
            TokenKind::Keyword(Keyword::String) => self.handle_assignment(Type::String, pos),
            TokenKind::Keyword(Keyword::Function) => self.handle_assignment(Type::Function, pos),
            TokenKind::Keyword(Keyword::Unit) => self.handle_assignment(Type::Unit, pos),
            TokenKind::Keyword(Keyword::List) => self.handle_assignment(Type::List, pos),
            _ => Err(Error::new(
                ErrorType::UnexpectedTokenType,
                pos,
                type_tok.original,
                Some("expected: type"),
            )),
        }
    }

    fn handle_assignment(&mut self, a_type: Type, pos: Position) -> Result<Assignment, Error> {
        let identifier = self.expect(|k| matches!(k, TokenKind::Identifier(_)))?;
        let ident_str = match identifier.kind {
            TokenKind::Identifier(s) => s,
            _ => unreachable!(),
        };
        self.expect(|k| matches!(k, TokenKind::Operator(Operator::Assign)))?;
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
            .ok_or_else(|| Error::generic_eof("expected an expression"))?;
        let pos = tok.position.clone();
        // simple expressions
        let mut lhs = match tok.kind {
            TokenKind::Literal(_) | TokenKind::Keyword(Keyword::True | Keyword::False) => {
                Expression::LiteralExp(self.parse_literal_token()?)
            }
            TokenKind::Identifier(_) => Expression::IdentifierExp(self.parse_identifier()?),
            TokenKind::Separator(Separator::LParen) => self.parse_paren_expr()?,
            TokenKind::Keyword(Keyword::Fn) => Expression::FunctionExp(self.parse_function()?),
            TokenKind::Keyword(Keyword::If) => Expression::IfExp(self.parse_if_expr()?),
            _ => {
                return Err(Error::new(
                    ErrorType::UnexpectedTokenType,
                    pos,
                    tok.original,
                    Some("expected literal or identifier"),
                ));
            }
        };
        // right-recursive descent (if operator is present)
        while let Some(tok) = self.peek() {
            // function calls
            if matches!(tok.kind, TokenKind::Separator(Separator::LParen)) {
                lhs = Expression::CallExp(self.parse_call(lhs)?);
                continue;
            }
            // check for operator
            let op = match &tok.kind {
                TokenKind::Operator(op) => op.clone(),
                // special case: cons
                TokenKind::Cons => {
                    let cons_prec = 1;
                    if cons_prec < min_prec {
                        break;
                    }
                    self.advance();
                    let rhs = self.parse_expression(cons_prec)?;
                    lhs = Expression::ConsExp(ConsExp::new(pos.clone(), lhs, rhs));
                    continue;
                }
                _ => break,
            };
            // handle binary opeartor
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

    fn parse_call(&mut self, callee: Expression) -> Result<Call, Error> {
        let tok = self.expect(|x| matches!(x, TokenKind::Separator(Separator::LParen)))?;
        let pos = tok.position.clone();
        // no args
        let mut args = Vec::new();
        if self.compare_kind(|x| matches!(x, TokenKind::Separator(Separator::RParen))) {
            self.advance();
            return Ok(Call {
                position: pos,
                callee: Box::new(callee),
                args,
            });
        }
        // args
        loop {
            args.push(Argument {
                position: pos.clone(),
                value: self.parse_expression(0)?,
            });
            if self.compare_kind(|x| matches!(x, TokenKind::Separator(Separator::Comma))) {
                self.advance();
                continue;
            }
            break;
        }
        self.expect(|x| matches!(x, TokenKind::Separator(Separator::RParen)))?;

        Ok(Call {
            position: pos,
            callee: Box::new(callee),
            args,
        })
    }

    fn parse_literal_token(&mut self) -> Result<Literal, Error> {
        let tok = self.advance().unwrap();
        let pos = tok.position.clone();
        if let TokenKind::Literal(val) = tok.kind {
            return Ok(Literal {
                position: pos,
                value: val.get_literal_value(),
            });
        }
        // booleans are keywords in the grammar
        if tok.kind == TokenKind::Keyword(Keyword::True) {
            return Ok(Literal {
                position: pos,
                value: LiteralValue::Bool(true),
            });
        }
        if tok.kind == TokenKind::Keyword(Keyword::False) {
            return Ok(Literal {
                position: pos,
                value: LiteralValue::Bool(false),
            });
        }
        Err(Error::generic_utt(tok))
    }

    fn parse_identifier(&mut self) -> Result<Identifier, Error> {
        let tok = self.expect(|x| matches!(x, TokenKind::Identifier(_)))?;
        let pos = tok.position.clone();
        Ok(Identifier {
            position: pos,
            name: tok.original,
        })
    }

    fn parse_function(&mut self) -> Result<Function, Error> {
        // consume fn
        let fn_keyword = self.advance().unwrap();
        let pos = fn_keyword.position.clone();
        // parse param list
        let params = self.parse_params()?;
        // parse return type
        self.expect(|x| matches!(x, TokenKind::Separator(Separator::Arrow)))?;
        let tok = self.expect(is_type)?;
        let returns = get_type_from_keyword(tok)?;
        self.expect(|x| matches!(x, TokenKind::Separator(Separator::LBrace)))?;
        let mut statement_list: Vec<Statement> = Vec::new();
        loop {
            if let Some(tok) = self.peek() {
                if matches!(tok.kind, TokenKind::Separator(Separator::RBrace)) {
                    self.advance(); // consume '}'
                    break;
                }
            } else {
                return Err(Error::generic_eof("expected closing '}' in function"));
            }
            statement_list.push(self.parse_statement()?);
        }
        // last statement must be something that can be returned
        let last = statement_list.last();
        if last.is_none() || matches!(last.unwrap(), Statement::Assignment(_)) {
            return Err(Error::new(
                ErrorType::FunctionShouldEndWithReturn,
                pos,
                // need to get variant name here
                &last.map_or("None".into(), |last| format!("{:?}", last)),
                Some("function must return"),
            ));
        }
        let last = last.unwrap();
        // if the last statement is an expression
        // convert it to an implicit return
        if let Statement::Expression(_) = last.clone() {
            let exp = statement_list.pop().unwrap().expect_expression()?.clone();
            statement_list.push(Statement::Return(Return {
                // position: exp.get_position().clone(),
                expression: exp.clone(),
            }));
        }
        Ok(Function {
            position: pos,
            params: params,
            returns: returns,
            body: StatementList {
                statements: statement_list,
            },
        })
    }

    fn parse_params(&mut self) -> Result<Vec<Param>, Error> {
        // match paren
        self.expect(|x| matches!(x, TokenKind::Separator(Separator::LParen)))?;
        let mut res = Vec::new();
        // case: no params
        if self.compare_kind(|x| matches!(x, TokenKind::Separator(Separator::RParen))) {
            self.advance();
            return Ok(res);
        }
        // loop: params until Rparen
        loop {
            res.push(self.parse_param()?);
            // case: finished
            if self.compare_kind(|x| matches!(x, TokenKind::Separator(Separator::RParen))) {
                self.advance();
                break;
            }
            // case: comma, go again
            if self.compare_kind(|x| matches!(x, TokenKind::Separator(Separator::Comma))) {
                self.advance();
                continue;
            }
            // case: anything else
            let tok = self
                .peek()
                .ok_or_else(|| Error::generic_eof("incomplete params list"))?;

            return Err(Error::new(
                ErrorType::UnexpectedTokenType,
                tok.position,
                tok.original,
                Some("expected valid params"),
            ));
        }
        Ok(res)
    }

    fn parse_param(&mut self) -> Result<Param, Error> {
        // match identifier -> colon -> type
        let id = self.expect(|x| matches!(x, TokenKind::Identifier(_)))?;
        self.expect(|x| matches!(x, TokenKind::Separator(Separator::Colon)))?;
        let type_token = self.expect(is_type)?;
        let ty = match type_token.kind {
            TokenKind::Keyword(Keyword::Bool) => Type::Bool,
            TokenKind::Keyword(Keyword::I32) => Type::I32,
            TokenKind::Keyword(Keyword::F32) => Type::F32,
            TokenKind::Keyword(Keyword::String) => Type::String,
            TokenKind::Keyword(Keyword::Function) => Type::Function,
            TokenKind::Keyword(Keyword::Unit) => Type::Unit,
            TokenKind::Keyword(Keyword::List) => Type::List,
            _ => return Err(Error::generic_utt(type_token)),
        };
        let pos = id.position.clone();
        let ident_str = if let TokenKind::Identifier(ref s) = id.kind {
            s.clone()
        } else {
            unreachable!();
        };
        Ok(Param {
            position: pos,
            param_type: ty,
            identifier: ident_str,
        })
    }

    fn parse_if_expr(&mut self) -> Result<IfExp, Error> {
        let if_tok = self.expect(|x| matches!(x, TokenKind::Keyword(Keyword::If)))?;
        let pos = if_tok.position.clone();

        // condition
        self.expect(|x| matches!(x, TokenKind::Separator(Separator::LParen)))?;
        let cond = self.parse_expression(0)?;
        self.expect(|x| matches!(x, TokenKind::Separator(Separator::RParen)))?;

        // (old) parse true branch
        // self.expect(|x| matches!(x, TokenKind::Separator(Separator::LBrace)))?;
        // let then_branch = self.parse_statement()?;
        // self.expect(|x| matches!(x, TokenKind::Separator(Separator::RBrace)))?;

        let mut res = IfExp {
            position: pos,
            if_cond: Box::new(cond),
            then_branch: Box::new(self.parse_branch()?),
            else_branch: None,
        };

        if self.optional(|x| matches!(x, TokenKind::Keyword(Keyword::Else))) {
            let else_branch = self.parse_branch()?;
            res.else_branch = Some(Box::new(else_branch));
        }

        // old parse else branch
        // if !self.compare_kind(|x| matches!(x, TokenKind::Keyword(Keyword::Else))) {
        //     return Ok(res);
        // }
        // self.advance();
        // self.expect(|x| matches!(x, TokenKind::Separator(Separator::LBrace)))?;
        // let else_branch = self.parse_statement()?;
        // self.expect(|x| matches!(x, TokenKind::Separator(Separator::RBrace)))?;
        // res.else_branch = Some(Box::new(else_branch));
        Ok(res)
    }

    fn parse_branch(&mut self) -> Result<Statement, Error> {
        let braces_present =
            self.optional(|x| matches!(x, TokenKind::Separator(Separator::LBrace)));
        let branch = self.parse_statement()?;
        if braces_present {
            self.expect(|x| matches!(x, TokenKind::Separator(Separator::RBrace)))?;
        }
        Ok(branch)
    }

    fn parse_paren_expr(&mut self) -> Result<Expression, Error> {
        let left = self.expect(|x| matches!(x, TokenKind::Separator(Separator::LParen)))?;
        // special case, unit literal
        if self.optional(|x| matches!(x, TokenKind::Separator(Separator::RParen))) {
            return Ok(Expression::LiteralExp(Literal {
                position: left.position,
                value: LiteralValue::Unit,
            }));
        }
        let expr = self.parse_expression(0)?;
        self.expect(|x| matches!(x, TokenKind::Separator(Separator::RParen)))?;
        Ok(Expression::ParenExp(Box::new(expr)))
    }

    /*
     * Utility functions
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

    // consumes and returns a required token
    fn expect<F>(&mut self, cond: F) -> Result<Token, Error>
    where
        F: Fn(&TokenKind) -> bool,
    {
        match self.advance() {
            Some(tok) if cond(&tok.kind) => Ok(tok),
            Some(tok) => Err(Error::new(
                ErrorType::UnexpectedTokenType,
                tok.position,
                format!("{:?}", tok.kind),
                None,
            )),
            None => Err(Error::generic_eof("unknown")),
        }
    }

    // like expect but optional
    // returns a boolean representing if the token was consumed
    fn optional<F>(&mut self, cond: F) -> bool
    where
        F: Fn(&TokenKind) -> bool,
    {
        match self.peek() {
            Some(tok) if cond(&tok.kind) => {
                self.pos += 1;
                true
            }
            _ => false,
        }
    }

    // like optional but does not consume
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

fn get_type_from_keyword(token: Token) -> Result<Type, Error> {
    let res = match token.kind {
        TokenKind::Keyword(Keyword::Bool) => Type::Bool,
        TokenKind::Keyword(Keyword::I32) => Type::I32,
        TokenKind::Keyword(Keyword::F32) => Type::F32,
        TokenKind::Keyword(Keyword::String) => Type::String,
        TokenKind::Keyword(Keyword::Function) => Type::Function,
        TokenKind::Keyword(Keyword::Unit) => Type::Unit,
        _ => {
            return Err(Error::new(
                ErrorType::UnexpectedTokenType,
                token.position,
                format!("{:?}", token.kind),
                Some("expected a token corresponding to a type"),
            ));
        }
    };
    Ok(res)
}

// returns true if token is a type
fn is_type(token: &TokenKind) -> bool {
    match token {
        TokenKind::Keyword(Keyword::Bool)
        | TokenKind::Keyword(Keyword::I32)
        | TokenKind::Keyword(Keyword::F32)
        | TokenKind::Keyword(Keyword::String)
        | TokenKind::Keyword(Keyword::Function)
        | TokenKind::Keyword(Keyword::List)
        | TokenKind::Keyword(Keyword::Unit) => true,
        _ => false,
    }
}
