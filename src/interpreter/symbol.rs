use std::collections::HashMap;

use crate::{parser::ast::Function, parser::ast::Literal};

// borrows from AST
pub enum SymbolValue {
    Literal(Literal),
    Function(Function),
}

pub struct Scope {
    symbols: HashMap<String, SymbolValue>,
}

impl Scope {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
        }
    }
    pub fn define(&mut self, name: String, value: SymbolValue) {
        self.symbols.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<&SymbolValue> {
        self.symbols.get(name)
    }
}
