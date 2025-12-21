/*
* Token type
* */

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

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    If,
    Else,
    Fn,
    I32,
    F32,
    Bool,
    String,
    True,
    False,
    Return,
    Function,
}
