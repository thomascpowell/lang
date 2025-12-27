use std::ops::{Add, Div, Mul, Sub};

use crate::{
    error_types::Error,
    interpreter::{closure::Closure, exec_result::ExecResult, symbol::Symbol},
    parser::ast::{Type},
    position::Position,
};

/**
* Value type
* */

#[derive(Clone, Debug)]
pub enum Value {
    Int(i32),
    Float(f32),
    Bool(bool),
    String(String),
    Function(Closure),
    NativeFunction(fn(Vec<Value>) -> Result<ExecResult, Error>),
    Uninitialized,
}

impl Value {
    pub fn expect_int(&self) -> Result<i32, Error> {
        if let Value::Int(x) = self {
            return Ok(*x);
        }
        Err(Error::generic_invalid_operand(self))
    }
    pub fn expect_float(&self) -> Result<f32, Error> {
        if let Value::Float(x) = self {
            return Ok(*x);
        }
        Err(Error::generic_invalid_operand(self))
    }

    pub fn expect_numeric(&self) -> Result<f32, Error> {
        match self {
            Value::Float(x) => Ok(*x),
            Value::Int(x) => Ok(*x as f32),
            _ => Err(Error::generic_invalid_operand(self)),
        }
    }

    pub fn expect_bool(&self) -> Result<bool, Error> {
        if let Value::Bool(x) = self {
            return Ok(*x);
        }
        Err(Error::generic_invalid_operand(self))
    }
    pub fn expect_string(&self) -> Result<String, Error> {
        if let Value::String(x) = self {
            return Ok(x.clone());
        }
        Err(Error::generic_invalid_operand(self))
    }
    pub fn expect_function(&mut self) -> Result<Closure, Error> {
        if let Value::Function(x) = self {
            return Ok(x.clone());
        }
        Err(Error::generic_invalid_operand(self))
    }

    pub fn into_symbol(self, pos: Position) -> Symbol {
        Symbol {
            pos: pos,
            ty: self.get_type(),
            val: self,
        }
    }

    pub fn get_type(&self) -> Type {
        match self {
            Value::Int(_) => Type::I32,
            Value::Float(_) => Type::F32,
            Value::Bool(_) => Type::Bool,
            Value::String(_) => Type::String,
            Value::NativeFunction(_) | Value::Function(_) => Type::Function,
            Value::Uninitialized => unreachable!(),
        }
    }

    pub fn display(&self) -> String {
        match self {
            Self::Int(i) => i.to_string(),
            Self::Float(i) => i.to_string(),
            Self::Bool(b) => if *b { "true" } else { "false" }.into(),
            Self::String(s) => s.clone(),
            Self::Function(_) => "[function]".to_string(),
            Self::NativeFunction(_) => "[native function]".to_string(),
            Value::Uninitialized => unreachable!(),
        }
    }
}

/**
* Operator implementations
* */

impl Mul for Value {
    type Output = Option<Value>;
    fn mul(self, rhs: Value) -> Option<Value> {
        match (self, rhs) {
            (Value::Int(a), Value::Int(b)) => Some(Value::Int(a * b)),
            (Value::Float(a), Value::Float(b)) => Some(Value::Float(a * b)),
            _ => None,
        }
    }
}

impl Div for Value {
    type Output = Option<Value>;
    fn div(self, rhs: Value) -> Option<Value> {
        match (self, rhs) {
            (Value::Int(a), Value::Int(b)) => Some(Value::Int(a / b)),
            (Value::Float(a), Value::Float(b)) => Some(Value::Float(a / b)),
            _ => None,
        }
    }
}

impl Add for Value {
    type Output = Option<Value>;
    fn add(self, rhs: Value) -> Option<Value> {
        match (self, rhs) {
            (Value::Int(a), Value::Int(b)) => Some(Value::Int(a + b)),
            (Value::Float(a), Value::Float(b)) => Some(Value::Float(a + b)),
            _ => None,
        }
    }
}

impl Sub for Value {
    type Output = Option<Value>;
    fn sub(self, rhs: Value) -> Option<Value> {
        match (self, rhs) {
            (Value::Int(a), Value::Int(b)) => Some(Value::Int(a - b)),
            (Value::Float(a), Value::Float(b)) => Some(Value::Float(a - b)),
            _ => None,
        }
    }
}
