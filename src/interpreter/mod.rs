use crate::{error_types::Error, interpreter::symbol::*, parser::ast::*};
pub mod symbol;

/**
* wip interpreter
* */

// this matches the general structure of the other components
pub fn interpret(ast: StatementList) -> Result<(), Error> {
    let mut interpreter = Interpreter::new(ast);
    while interpreter.has_next() {
        interpreter.interpret_statement()?;
        interpreter.advance()
    }
    Ok(())
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
     * Interpreter logic
     * */

    pub fn interpret_statement(&mut self) -> Result<(), Error> {
        let stmt = self.peek().unwrap();
        return match stmt {
            // cannot pass stmt contents to self.interpret_x()
            // because it is an interal reference and the methods borrow self
            Statement::Assignment(_) => self.interpret_assignment(),
            Statement::Expression(_) => self.interpret_expression(),
            Statement::Return(_) => self.interpret_return(),
        }
    }

    fn interpret_assignment(&mut self) -> Result<(), Error> {
        let asn = self.peek().unwrap().expect_assignment()?;
        todo!()
    }
    fn interpret_expression(&mut self) -> Result<(), Error> {
        let exp = self.peek().unwrap().expect_assignment()?;
        todo!()
    }
    fn interpret_return(&mut self) -> Result<(), Error> {
        let ret = self.peek().unwrap().expect_assignment()?;
        todo!()
    }

    /*
     * Utility functions
     * */

    fn has_next(&mut self) -> bool {
        self.peek().is_some()
    }

    fn peek(&self) -> Option<&Statement> {
        self.ast.statements.get(self.pos)
    }

    fn advance(&mut self) {
        self.pos += 1
    }
}
