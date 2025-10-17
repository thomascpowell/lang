/*
* Token data
* */

// metadata
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub original: String,
    pub line: usize,
    pub col: usize,
}

// meaning
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
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
    Mul,
    Div,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
    Assign,
    Not
}

#[derive(Debug, Clone, PartialEq)]
pub enum Separator {
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    Semicolon,
}
pub const OPEN_SEPARATORS: [char; 2] = ['(', '{'];
pub const CLOSE_SEPARATORS: [char; 2] = [')', '}'];


#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Int(i32),
    Bool(bool),
    String(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    Fn,
    I32,
    Bool,
    String,
    True,
    False,
}
