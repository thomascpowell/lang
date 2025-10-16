/*
* Token data
* */

#[derive(Debug, Clone, PartialEq)]
pub struct TokenData {
    pub kind: Token,
    pub original: String,
    pub line: usize,
    pub col: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Identifier(String),
    Keyword(Keyword),
    Separator(Separator),
    Operator(Operator),
    Literal(Literal),
    Comment(String),
}

/*
* Token subtypes
* */

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mult,
    Div,
    Eq,
    Ne,
    Lt,
    LtEq,
    Gt,
    GtEq,
    And,
    Or,
    Assign,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Separator {
    LParen,
    RParen,
    LBrace,
    Rbrace,
    Comma,
    Semicolon,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Int(i32),
    Uint(u32),
    Bool(bool),
    String(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    Fn,
    I32,
    U32,
    Bool,
    String,
    True,
    False,
}
