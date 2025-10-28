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

pub struct Return {
    pub position: Position,
    pub expression: Expression,
}

pub struct Function {
    pub position: Position,
    pub args: Vec<Param>,
    pub body: StatementList,
}
pub struct Param {
    // for def
    pub position: Position,
    pub param_type: Type,
    pub identifier: String,
}

pub struct Call {
    pub position: Position,
    // any expression can be called
    pub callee: Box<Expression>,
    pub args: Vec<Argument>,
}
pub struct Argument {
    // for call
    pub position: Position,
    pub value: Expression,
}

pub struct BinaryExp {
    pub position: Position,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
    pub operator: Operator,
}

pub struct IfExp {
    // if is an expression
    pub if_cond: Box<Expression>,
    pub then_branch: Box<Statement>,
    pub else_branch: Option<Box<Statement>>,
}

/*
* Keywords & Operators
* */

pub enum Type {
    I32,
    String,
    Bool,
}

pub type Operator = crate::lexer::token::Operator;

/*
* Metadata
* */

#[derive(Clone)]
pub struct Position {
    pub start_line: usize,
    pub start_col: usize,
}
