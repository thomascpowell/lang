use std::{collections::HashMap};

use crate::{
    error_types::{Error, ErrorType},
    parser::ast::{Function, Literal},
};

// borrows from AST
pub enum Symbol {
    Literal(Literal),
    Function(Function),
}

pub struct Scope {
    symbols: HashMap<String, Symbol>,
}

impl Scope {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
        }
    }
    pub fn define(&mut self, name: String, value: Symbol) {
        self.symbols.insert(name, value);
    }
    pub fn get(&self, name: &str) -> Option<&Symbol> {
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

    fn get_symbol(&mut self, identifier: String) -> Result<&Symbol, Error> {
        // start with most recent/specific scope
        // look for symbol
        for scope in self.scopes.iter().rev() {
            if let Some(sym) = scope.get(&identifier) {
                return Ok(sym);
            }
        }
        // no symbol -> error
        Err(Error {
            error_type: ErrorType::InvalidSymbol,
            start_line: 0,
            start_col: 0,
            found: identifier,
            message: Some("invalid identifer name".to_string()),
        })
    }
}
