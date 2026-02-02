use crate::{parser::ast::LiteralValue, position::Position};

/*
* Token type
* */

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub original: String,
    pub position: Position,
}

impl Token {
    pub fn display(&self) -> String {
        format!("{:?} - {}", self.kind, self.original)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Identifier(String),
    Keyword(Keyword),
    Separator(Separator),
    Operator(Operator),
    Literal(Literal),

    // comments are thrown out atm
    Comment(String),

    // cons is a _constructor_
    Cons
}

/*
* Token subtypes
* */

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
    Assign,
    Not,
}
impl Operator {
    pub fn get_precedence(&self) -> u8 {
        match self {
            Operator::Or => 1,
            Operator::And => 2,
            Operator::Eq | Operator::Ne => 3,
            Operator::Lt | Operator::Le | Operator::Gt | Operator::Ge => 4,
            Operator::Add | Operator::Sub => 5,
            Operator::Mul | Operator::Div | Operator::Mod => 6,
            // if cons was here it would be very low precedence
            _ => 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Separator {
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    Semicolon,
    Colon,
    Arrow,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Int(i32),
    Float(f32),
    Bool(bool),
    String(String),
}

impl Literal {
    // returns the AST node equivilant of Literal
    pub fn get_literal_value(self) -> LiteralValue {
        match self {
            Self::Int(x) => LiteralValue::Int(x),
            Self::Float(x) => LiteralValue::Float(x),
            Self::Bool(x) => LiteralValue::Bool(x),
            Self::String(x) => LiteralValue::String(x),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    // control flow
    If,
    Else,
    // function-related
    Fn,
    Return,
    // literals
    True,
    False,
    // types
    I32,
    F32,
    Bool,
    String,
    Function,
    Unit,
}
