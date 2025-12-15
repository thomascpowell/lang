use crate::{interpreter::symbol::Value, lexer::token::Token, parser::ast::Statement};

/**
* Error Types
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

    /**
     * Functions for creating common errors
     * */

    pub fn generic_utt(tok: Token) -> Self {
        Error::new(
            ErrorType::UnexpectedTokenType,
            tok.line,
            tok.col,
            "unexpected token",
            None,
        )
    }
    pub fn generic_se(identifier: String) -> Self {
        Error::new(
            ErrorType::UnexpectedTokenType,
            0,
            0,
            identifier,
            Some("unexpected error setting identifier"),
        )
    }
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
    pub fn generic_invalid_operand(operand: &Value) -> Self {
        let operator_type: &str = match operand {
            Value::Int(_) => "i32",
            Value::Float(_) => "f32",
            Value::Bool(_) => "bool",
            Value::String(_) => "string",
            Value::Function(_) => "function",
            Value::NativeFunction(_) => " native function",
        };
        return Error::new(
            ErrorType::InvalidOperand,
            0,
            0,
            operator_type,
            Some("check operator and operand types"),
        );
    }
    pub fn generic_invalid_params(param_count: usize, message: &str) -> Self {
        return Error::new(
            ErrorType::InvalidParams,
            0,
            0,
            param_count.to_string() + " parameters",
            Some(message),
        );
    }
    pub fn generic_eof(expected: &str) -> Self {
        Error::new(ErrorType::UnexpectedEOF, 0, 0, "EOF", Some(expected))
    }
    pub fn generic() -> Self {
        Error::new(ErrorType::Default, 0, 0, "unknown. likely incomplete", None)
    }
    pub fn generic_message(ty: ErrorType, message: String) -> Self {
        Error::new(ty, 0, 0, message, None)
    }
}
