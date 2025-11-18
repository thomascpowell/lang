use std::{collections::HashMap, rc::Rc};

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
    pub fn expect_unit(&mut self) -> Result<(), Error> {
        todo!()
    }
    pub fn expect_value(&mut self) -> Result<Value, Error> {
        todo!()
    }
    pub fn expect_returned(&mut self) -> Result<Value, Error> {
        todo!()
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
    // a runtime function value
    Function(Rc<FunctionValue>),
}

#[derive(Clone)]
pub struct FunctionValue {
    pub ast: Function,
    pub env: usize,
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
