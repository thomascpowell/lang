use crate::{
    interpreter::value::Value, lexer::token::Token, parser::ast::Statement, position::Position,
};

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

    pub fn generic_uer() -> Self {
        Error::new(
            ErrorType::UnexpectedExecResult,
            Position { line: 0, col: 0 },
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
            Value::Uninitialized => unreachable!(),
        };
        return Error::new(
            ErrorType::InvalidOperand,
            Position { line: 0, col: 0 },
            operator_type,
            Some("check operator and operand types"),
        );
    }
    pub fn generic_eof(expected: &str) -> Self {
        Error::new(
            ErrorType::UnexpectedEOF,
            Position { line: 0, col: 0 },
            "EOF",
            Some(expected),
        )
    }
    pub fn generic() -> Self {
        Error::new(
            ErrorType::Default,
            Position { line: 0, col: 0 },
            "unknown",
            None,
        )
    }
    pub fn generic_message(ty: ErrorType, message: String) -> Self {
        Error::new(ty, Position { line: 0, col: 0 }, message, None)
    }
}
