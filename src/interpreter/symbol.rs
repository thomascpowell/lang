use crate::{
    error_types::Error,
    interpreter::exec_result::ExecResult,
    parser::ast::{Function, Type},
};

/**
* Symbol type
* */

pub struct Symbol {
    pub ty: Type,
    pub val: Value,
}

#[derive(Clone, Debug)]
pub enum Value {
    Int(i32),
    Bool(bool),
    String(String),
    Function(Function),
    NativeFunction(fn(Vec<Value>) -> Result<ExecResult, Error>),
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

                // nativefn does not have a type
                // so it cannot be a symbol
                // this is fine afik
                Value::NativeFunction(_) => panic!(),
            },
            val: self,
        }
    }

    pub fn display(&self) -> String {
        match self {
            Self::Int(i) => i.to_string(),
            Self::Bool(b) => if *b { "true" } else { "false" }.into(),
            Self::String(s) => s.clone(),
            Self::Function(_) => "[function]".to_string(),
            Self::NativeFunction(_) => "[native function]".to_string(),
        }
    }
}
