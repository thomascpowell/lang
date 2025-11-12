use crate::{
    error_types::Error,
    interpreter::symbol::ScopeStack,
    parser::ast::{Statement, StatementList},
};
pub mod symbol;

/**
* wip interpreter
* */

pub fn interpret(ast: StatementList) -> Result<(), Error> {
    let mut interpreter = Interpreter::new(ast);
    while interpreter.has_next() {}
    todo!()
}

struct Interpreter {
    pub ast: StatementList,
    pub pos: usize,
    pub scopes: ScopeStack,
}

impl Interpreter {
    fn new(ast: StatementList) -> Self {
        Interpreter {
            ast: ast,
            pos: 0,
            scopes: ScopeStack::new(),
        }
    }

    /*
     * Utility functions
     * */

    fn has_next(&mut self) -> bool {
        self.peek().is_some()
    }

    fn peek(&self) -> Option<Statement> {
        self.ast.statements.get(self.pos).cloned()
    }
}
