use crate::{error_types::Error, interpreter::symbol::Scope, parser::ast::StatementList};
pub mod symbol;

// TODO: interpreter
pub fn interpret(ast: StatementList) -> Result<(), Error> {
    todo!()
}

struct Interpreter {
    ast: Vec<StatementList>,
    pos: usize,
    scopes: Vec<Scope>,
}

