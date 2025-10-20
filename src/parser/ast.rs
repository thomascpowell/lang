use crate::lexer::token::*;

/*
* Nonterminal types
* */

pub struct StatementList {
    pub statements: Vec<Statement>,
}

pub enum Statement {
    Assignment(Assignment),
    Expression(Expression),
    Return(Return),
}

pub enum Expression {
    Literal(Token),
    Identifier(Token),
    Function(Function),
    Call(Call),
    BinaryExp(BinaryExp),
    IfExp(IfExp),
    ParenExp(Box<Expression>),
}


pub struct Assignment {
    pub position: Position,
    pub assignment_type: Type,
    pub identifier: String,
    pub expression: Expression,
}


pub struct Return {}

pub struct Function {}

pub struct Call {}

pub struct BinaryExp {}

pub struct IfExp {}

/*
* Keywords
* */

pub enum Type {
    I32,
    String,
    Bool,
}

/*
* Metadata
* */

pub struct Position {
    start_line: usize,
    start_col: usize,
}
