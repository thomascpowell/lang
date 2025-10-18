#[derive(Debug)]
pub enum LexerErrorType {
    InvalidChar,
    UnterminatedStringLiteral,
    InvalidIntLiteral,
    InvalidOperator,
    UnexpectedEOF,
    Default,
}

#[derive(Debug)]
pub struct LexerError {
    pub error_type: LexerErrorType,
    pub start_line: usize,
    pub start_col: usize,
    pub found: String,
    pub message: Option<String>,
}

impl LexerError {
    pub fn new(
        error_type: LexerErrorType,
        start_line: usize,
        start_col: usize,
        found: impl Into<String>,
        message: Option<&str>,
    ) -> Self {
        Self {
            error_type,
            start_line,
            start_col,
            found: found.into(),
            message: message.map(|m| m.into()),
        }
    }

    pub fn display(&self) -> String {
        format!(
            "---\nerror: {:?} at line {}, col {}\nfound: '{}'\ninfo: {}\n---",
            self.error_type,
            self.start_line,
            self.start_col,
            self.found,
            self.message.as_deref().unwrap_or("none"),
        )
    }
}
