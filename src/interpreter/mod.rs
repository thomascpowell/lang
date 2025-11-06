use crate::{
    error_types::{Error, ErrorType},
    interpreter::symbol::{Scope, ScopeStack, Symbol},
    parser::ast::{Statement, StatementList},
};
pub mod symbol;

pub fn interpret(ast: StatementList) -> Result<(), Error> {
    let mut interpreter = Interpreter::new(ast);
    todo!()
}

struct Interpreter {
    pub ast: StatementList,
    pub pos: usize,
    pub scopes: ScopeStack
}

impl Interpreter {
    fn new(ast: StatementList) -> Self {
        Interpreter {
            ast: ast,
            pos: 0,
            scopes: ScopeStack::new(),
        }
    }
    fn peek(&mut self) -> Statement {
        todo!()
    }

}
