#[derive(Debug)]
pub enum LexerError {
    InvalidChar(String),
    UnterminatedStringLiteral(String),
    InvalidIntLiteral(String),
    InvalidOperator(String),
    UnexpectedEOF(String),
    Default
}
