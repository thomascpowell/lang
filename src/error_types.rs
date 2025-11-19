use crate::{
    lexer::token::Token,
    parser::ast::{Position, Statement},
};

#[derive(Debug)]
pub enum ErrorType {
    // Lexer
    InvalidChar,
    UnterminatedStringLiteral,
    InvalidIntLiteral,
    InvalidOperator,
    UnexpectedEOF,
    // Parser
    UnexpectedTokenType,
    FunctionShouldEndWithReturn,
    // Interpreter
    InvalidSymbol,
    UnexpectedStatementType,
    InvalidReturnLocation,
    UnexpectedExecResult,
    // Shared
    Default,
}

#[derive(Debug)]
pub struct Error {
    pub error_type: ErrorType,
    pub start_line: usize,
    pub start_col: usize,
    pub found: String,
    pub message: Option<String>,
}

impl Error {
    pub fn new(
        error_type: ErrorType,
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

    // lazy error - unexpected token type
    pub fn generic_utt(tok: Token) -> Self {
        Error::new(
            ErrorType::UnexpectedTokenType,
            tok.line,
            tok.col,
            "unexpected token",
            None,
        )
    }

    // unexpected scope error
    pub fn generic_se(identifier: String) -> Self {
        Error::new(
            ErrorType::UnexpectedTokenType,
            0,
            0,
            identifier,
            Some("unexpected error setting identifier"),
        )
    }

    // unexpected statement type
    // uses recursive Statement::get_position() function
    pub fn generic_ust(stmt: &Statement) -> Self {
        let pos = stmt.get_position();
        Error::new(
            ErrorType::UnexpectedStatementType,
            pos.start_line,
            pos.start_col,
            "unexpected statement type",
            None,
        )
    }

    pub fn generic_uer() -> Self {
        Error::new(
            ErrorType::UnexpectedExecResult,
            0,
            0,
            "unexpected exec result",
            None,
        )
    }

    // this happens rather often
    // dont have the location because thats in the token
    // the token that does not exist
    pub fn generic_eof(expected: &str) -> Self {
        Error::new(ErrorType::UnexpectedEOF, 0, 0, "EOF", Some(expected))
    }

    // this also happens rather often
    // mainly when im not done
    pub fn generic() -> Self {
        Error::new(ErrorType::Default, 0, 0, "unknown. likely incomplete", None)
    }

    pub fn generic_message(ty: ErrorType, message: String) -> Self {
        Error::new(ty, 0, 0, message, None)
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
