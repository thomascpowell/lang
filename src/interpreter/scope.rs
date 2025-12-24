use crate::{
    error_types::{Error, ErrorType},
    interpreter::{stdlib::*, symbol::Symbol, value::Value},
    parser::ast::Type,
    position::Position,
};
use std::{collections::HashMap, iter::zip, rc::Rc};

/**
* Scope type
* */

#[derive(Clone)]
pub struct Scope {
    pub symbols: HashMap<String, Symbol>,
    pub parent: Option<Rc<Scope>>,
}

pub fn get_stdlib_scope() -> Rc<Scope> {
    let mut new_scope = Scope {
        symbols: HashMap::new(),
        parent: None,
    };
    let pos = Position { col: 0, line: 0 };
    let names = vec!["floor", "print", "println", "panic", "read"];
    let functions = vec![std_floor, std_print, std_println, std_panic, std_read];
    for (name, function) in zip(names, functions) {
        let symbol = Symbol {
            pos: pos.clone(),
            ty: Type::Function,
            val: Value::NativeFunction(function),
        };
        new_scope.symbols.insert(name.into(), symbol);
    }
    return Rc::new(new_scope);
}

impl Scope {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
            parent: None,
        }
    }

    // creates a new scope with itself as the parent
    pub fn extend(self: &Rc<Scope>, name: String, symbol: Symbol) -> Rc<Scope> {
        let mut new_scope = Scope {
            symbols: HashMap::new(),
            parent: Some(Rc::clone(self)),
        };
        new_scope.symbols.insert(name, symbol);
        Rc::new(new_scope)
    }

    pub fn extend_many(self: &Rc<Scope>, binds: Vec<(String, Symbol)>) -> Rc<Scope> {
        let mut new_scope = Scope {
            symbols: HashMap::new(),
            parent: Some(Rc::clone(self)),
        };
        for (name, symbol) in binds.iter() {
            new_scope.symbols.insert(name.clone(), symbol.clone());
        }
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
            start_line: pos.line,
            start_col: pos.col,
            found: identifier.to_string(),
            message: Some("identifier name not found".to_string()),
        })
    }
}
