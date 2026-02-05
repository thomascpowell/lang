use crate::{
    interpreter::{stdlib::*, symbol::Symbol, value::Value},
    lang_error::{Error, ErrorType},
    parser::ast::Type,
    position::Position,
};
use std::{cell::RefCell, collections::HashMap, iter::zip, rc::Rc};

/**
* Scope type
* */

#[derive(Clone, Debug)]
pub struct Scope {
    pub symbols: HashMap<String, Rc<RefCell<Symbol>>>,
    pub parent: Option<Rc<Scope>>,
}

pub fn get_stdlib_scope() -> Rc<Scope> {
    let mut new_scope = Scope {
        symbols: HashMap::new(),
        parent: None,
    };
    let pos = Position { col: 0, line: 0 };
    // TODO: this could probably be a macro?
    let names = vec![
        "floor", "print", "println", "panic", "read", "assert", "new_list", "head", "tail",
        "length",
    ];
    let functions = vec![
        std_floor,
        std_print,
        std_println,
        std_panic,
        std_read,
        std_assert,
        std_new_list,
        std_head,
        std_tail,
        std_length,
    ];
    for (name, function) in zip(names, functions) {
        let symbol = Symbol {
            pos: pos.clone(),
            ty: Type::Function,
            val: Value::NativeFunction(function),
        };
        new_scope
            .symbols
            .insert(name.into(), Rc::new(RefCell::new(symbol)));
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
    pub fn extend(self: &Rc<Scope>, name: String, symbol: Rc<RefCell<Symbol>>) -> Rc<Scope> {
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
            new_scope
                .symbols
                .insert(name.clone(), Rc::new(RefCell::new(symbol.clone())));
        }
        Rc::new(new_scope)
    }

    // gets the requested symbol (read only)
    pub fn get_symbol(&self, identifier: &str, pos: Position) -> Result<Symbol, Error> {
        if let Some(symbol) = self.symbols.get(identifier) {
            return Ok(symbol.borrow().clone());
        }
        if let Some(parent) = &self.parent {
            return parent.get_symbol(identifier, pos);
        }
        Err(Error::new(ErrorType::InvalidSymbol, pos, identifier, None))
    }

    // gets the RefCell corresponding to the identifier
    pub fn get_symbol_cell(
        &self,
        identifier: &str,
        pos: Position,
    ) -> Result<Rc<RefCell<Symbol>>, Error> {
        if let Some(cell) = self.symbols.get(identifier) {
            return Ok(Rc::clone(cell));
        }
        if let Some(parent) = &self.parent {
            return parent.get_symbol_cell(identifier, pos);
        }
        Err(Error::new(ErrorType::InvalidSymbol, pos, identifier, None))
    }
}
