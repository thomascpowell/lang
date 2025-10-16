#[derive(Debug)]
pub enum LexerError {
    InvalidChar(String),
    UnterminatedStringLiteral(String),
    InvalidOperator(String),
    UnexpectedEOF,
    Default
}
