use crate::{
    error_types::Error,
    interpreter::symbol::Scope,
    parser::ast::{Statement, StatementList},
};
pub mod symbol;

// TODO: interpreter
pub fn interpret(ast: StatementList) -> Result<(), Error> {
    let mut interpreter = Interpreter::new(ast);
    todo!()
}

struct Interpreter {
    pub ast: StatementList,
    pub pos: usize,
    pub scopes: Vec<Scope>,
}

impl Interpreter {
    fn new(ast: StatementList) -> Self {
        Interpreter {
            ast: ast,
            pos: 0,
            scopes: Vec::new(),
        }
    }

    fn peek(&mut self) -> Statement {
        todo!()
    }
}
