use std::collections::HashMap;
use crate::interpreter::symbol::Symbol;

/**
* Scope type
* */

pub struct Scope {
    symbols: HashMap<String, Symbol>,
}

impl Scope {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: &str, value: Symbol) {
        self.symbols.insert(name.to_string(), value);
    }

    pub fn get(&self, name: &str) -> Option<&Symbol> {
        self.symbols.get(name)
    }
}
