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

impl Token {
    pub fn display(&self) -> String {
        format!("{:?} - {}", self.kind, self.original)
    }
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
    Not,
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
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Int(i32),
    Bool(bool),
    String(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    If,
    Else,
    Fn,
    I32,
    Bool,
    String,
    True,
    False,
}
