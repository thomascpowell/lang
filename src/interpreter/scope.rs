use crate::{
    error_types::{Error, ErrorType},
    interpreter::symbol::Symbol,
    parser::ast::Position,
};
use std::{collections::HashMap, rc::Rc};

/**
* Scope type
* */

#[derive(Clone)]
pub struct Scope {
    pub symbols: HashMap<String, Symbol>,
    pub parent: Option<Rc<Scope>>,
}

impl Scope {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
            parent: None,
        }
    }

    // creates a new scope with itself as the parent
    // TODO: multiple bindings at once
    pub fn extend(self: &Rc<Scope>, name: String, symbol: Symbol) -> Rc<Scope> {
        let mut new_scope = Scope {
            symbols: HashMap::new(),
            parent: Some(Rc::clone(self)),
        };
        new_scope.symbols.insert(name, symbol);
        Rc::new(new_scope)
    }

    pub fn get_symbol(&self, identifier: &str, pos: Position) -> Result<Symbol, Error> {
        if let Some(symbol) = self.symbols.get(identifier) {
            return Ok(symbol.clone());
        }
        if let Some(parent) = &self.parent {
            return parent.get_symbol(identifier, pos);
        }
        Err(Error {
            error_type: ErrorType::InvalidSymbol,
            start_line: pos.start_line,
            start_col: pos.start_col,
            found: identifier.to_string(),
            message: Some("identifier name not found".to_string()),
        })
    }
}
