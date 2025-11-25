use crate::{
    error_types::{Error},
    parser::ast::{Function, Type},
};

pub struct Symbol {
    pub ty: Type,
    pub val: Value,
}

#[derive(Clone)]
pub enum Value {
    Int(i32),
    Bool(bool),
    String(String),
    Function(Function),
}

impl Value {
    pub fn expect_int(&self) -> Result<i32, Error> {
        if let Value::Int(x) = self {
            return Ok(*x);
        }
        Err(Error::generic_invalid_operand(self))
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
    pub fn expect_function(&mut self) -> Result<Function, Error> {
        if let Value::Function(x) = self {
            return Ok(x.clone());
        }
        Err(Error::generic_invalid_operand(self))
    }

    pub fn into_symbol(self) -> Symbol {
        Symbol {
            ty: match &self {
                Value::Int(_) => Type::I32,
                Value::Bool(_) => Type::Bool,
                Value::String(_) => Type::String,
                Value::Function(f) => f.returns.clone(),
            },
            val: self,
        }
    }
} 

