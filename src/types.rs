#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Identifier,
    Keyword,
    Separator,
    Operator,
    Literal,
    Comment,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub value: String,
    pub line: usize,
    pub col: usize,
}

pub enum TokenTypes {
    Identifier,
    Keyword,
    Separator,
    Operator,
    Literal,
    Comment,
}

