use std::{any::type_name, collections::HashMap};

use crate::{
    error_types::{Error, ErrorType},
    parser::ast::{Function, Type},
};

pub enum ExecResult {
    Unit,            // statement produces nothing
    Value(Value),    // expression result
    Returned(Value), // function return
}

impl ExecResult {
    pub fn expect_unit(&self) -> Result<(), Error> {
        if matches!(self, ExecResult::Unit) {
            return Ok(());
        }
        Err(Error::generic_uer())
    }
    pub fn expect_value(&self) -> Result<Value, Error> {
        if let ExecResult::Value(v) = self {
            return Ok(v.clone());
        }
        Err(Error::generic_uer())
    }
    pub fn expect_returned(&self) -> Result<Value, Error> {
        if let ExecResult::Returned(r) = self {
            return Ok(r.clone());
        }
        Err(Error::generic_uer())
    }
}

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
    pub fn expect_int(&mut self) -> Result<i32, Error> {
        if let Value::Int(x) = self {
            return Ok(x.clone())
        }
        Err(Error::generic_invalid_operand(self))
    }
    pub fn expect_bool(&mut self) -> Result<bool, Error> {
        if let Value::Bool(x) = self {
            return Ok(x.clone())
        }
        Err(Error::generic_invalid_operand(self))
    }
    pub fn expect_string(&mut self) -> Result<String, Error> {
        if let Value::String(x) = self {
            return Ok(x.clone())
        }
        Err(Error::generic_invalid_operand(self))
    }
    pub fn expect_function(&mut self) -> Result<Function, Error> {
        if let Value::Function(x) = self {
            return Ok(x.clone())
        }
        Err(Error::generic_invalid_operand(self))
    }
}

pub struct Scope {
    symbols: HashMap<String, Symbol>,
}

impl Scope {
    fn new() -> Self {
        Self {
            symbols: HashMap::new(),
        }
    }
    fn define(&mut self, name: &str, value: Symbol) {
        self.symbols.insert(name.to_string(), value);
    }
    fn get(&self, name: &str) -> Option<&Symbol> {
        self.symbols.get(name)
    }
}

pub struct ScopeStack {
    scopes: Vec<Scope>,
}

impl ScopeStack {
    pub fn new() -> Self {
        let mut s = ScopeStack { scopes: Vec::new() };
        s.push_scope(); // add global scope
        s
    }

    pub fn get_symbol(&self, identifier: &str) -> Result<&Symbol, Error> {
        // start with most recent/specific scope
        // look for symbol
        // this is dynamic scoping (kinda bad)
        for scope in self.scopes.iter().rev() {
            if let Some(symbol) = scope.get(identifier) {
                return Ok(symbol);
            }
        }
        // no symbol -> error
        Err(Error {
            error_type: ErrorType::InvalidSymbol,
            start_line: 0,
            start_col: 0,
            found: identifier.to_string(),
            message: Some("identifier name not found".to_string()),
        })
    }

    // sets a new symbol at the top of the stack
    pub fn set_symbol(&mut self, identifier: &str, symbol: Symbol) -> Result<(), Error> {
        if let Some(scope) = self.scopes.last_mut() {
            scope.define(identifier, symbol);
            return Ok(());
        }
        Err(Error::generic_se(identifier.to_string()))
    }

    // adds a scope to the stack
    pub fn push_scope(&mut self) {
        self.scopes.push(Scope::new());
    }
    // removes a scope from the stack
    pub fn pop_scope(&mut self) {
        self.scopes.pop();
    }
}
