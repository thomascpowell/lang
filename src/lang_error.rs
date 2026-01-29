use crate::{
    interpreter::{exec_result::ExecResult, value::Value},
    lexer::token::Token,
    parser::ast::Statement,
    position::Position,
};

/**
* Error Type
* */

#[derive(Debug)]
pub enum ErrorType {
    // Lexer
    InvalidChar,
    UnterminatedStringLiteral,
    InvalidIntLiteral,
    InvalidFloatLiteral,
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
    InvalidOperand,
    InvalidParams,
    InvalidFunctionBody,
    TypeMismatch,
    // Stdlib
    StdRead,
    StdMissingArgs,
    StdAssertionFailure,
    // Shared
    Default,
}

#[derive(Debug)]
pub struct Error {
    pub error_type: ErrorType,
    pub position: Position,
    pub found: String,
    pub message: Option<String>,
}

const NO_POSITION: Position = Position { col: 0, line: 0 };

impl Error {
    pub fn new(
        error_type: ErrorType,
        position: Position,
        found: impl Into<String>,
        message: Option<&str>,
    ) -> Self {
        Self {
            error_type,
            position,
            found: found.into(),
            message: message.map(|m| m.into()),
        }
    }

    pub fn display(&self) -> String {
        format!(
            "---\nerror: {:?} at line {}, col {}\nfound: '{}'\ninfo: {}\n---",
            self.error_type,
            self.position.line,
            self.position.col,
            self.found,
            self.message.as_deref().unwrap_or("none"),
        )
    }

    /**
     * Functions for creating common errors
     * */

    pub fn generic_utt(tok: Token) -> Self {
        Error::new(
            ErrorType::UnexpectedTokenType,
            tok.position,
            "unexpected token",
            None,
        )
    }

    pub fn generic_ust(stmt: &Statement) -> Self {
        let pos = stmt.get_position();
        Error::new(
            ErrorType::UnexpectedStatementType,
            pos.clone(),
            "unexpected statement type",
            None,
        )
    }

    pub fn generic_uer(found: &ExecResult, expected: &str) -> Self {
        let msg = Some(format!("expected {}", expected));
        let found = format!("{:?}", found);
        Error {
            error_type: ErrorType::UnexpectedExecResult,
            position: NO_POSITION,
            found: found,
            message: msg,
        }
    }

    pub fn generic_invalid_operand(operand: &Value) -> Self {
        let op = format!("{:?}", operand.get_type());
        return Error::new(ErrorType::InvalidOperand, NO_POSITION, op, None);
    }

    pub fn generic_eof(expected: &str) -> Self {
        Error::new(ErrorType::UnexpectedEOF, NO_POSITION, "EOF", Some(expected))
    }

    pub fn generic() -> Self {
        Error::new(ErrorType::Default, NO_POSITION, "unknown", None)
    }

    pub fn generic_message(ty: ErrorType, message: String) -> Self {
        Error::new(ty, NO_POSITION, message, None)
    }
}
