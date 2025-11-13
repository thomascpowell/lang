use std::{collections::HashMap};

use crate::{
    error_types::{Error, ErrorType},
    parser::ast::{Function, Literal, Type},
};

// borrows from AST
pub struct Symbol {
    pub ty: Type,
    pub val: SymbolValue,
}
pub enum SymbolValue {
    Literal(Literal),
    Function(Function),
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
        Self { scopes: Vec::new() }
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
            message: Some("identifer name not found".to_string()),
        })
    }

    // sets a new symbol at the top of the stack
    pub fn set_symbol(&mut self, identifier: &str, symbol: Symbol) -> Result<(), Error> {
        if let Some(scope) = self.scopes.last_mut() {
            scope.define(identifier, symbol);
            return Ok(())
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
