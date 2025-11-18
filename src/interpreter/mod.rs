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
    pub call_depth: usize,
}

impl Interpreter {
    fn new(ast: StatementList) -> Self {
        Interpreter {
            ast: ast,
            pos: 0,
            scopes: ScopeStack::new(),
            call_depth: 0,
        }
    }

    /*
     * Interpreter logic
     * */

    pub fn interpret_statement(&mut self) -> Result<ExecResult, Error> {
        let stmt = self.peek().unwrap();
        return match stmt {
            // cannot pass stmt contents to self.interpret_x()
            // because it is an interal reference and the methods borrow self
            Statement::Assignment(_) => self.interpret_assignment(),
            Statement::Expression(_) => self.interpret_expression(),
            Statement::Return(_) => self.interpret_return(),
        };
    }

    fn interpret_assignment(&mut self) -> Result<ExecResult, Error> {
        // get parts of the assignment
        let asn = self.peek().unwrap().expect_assignment()?;
        let assignment_type = asn.assignment_type.clone();
        let identifier = asn.identifier.clone();

        let symbol_value = self
            .handle_expression(asn.expression.clone())?
            .expect_value()?;
        let symbol = Symbol {
            ty: assignment_type,
            val: symbol_value,
        };
        // push to the scope stack
        self.scopes.set_symbol(&identifier, symbol)?;
        Ok(ExecResult::Unit)
    }

    fn interpret_expression(&mut self) -> Result<ExecResult, Error> {
        let exp = self.peek().unwrap().expect_expression()?.clone();
        self.handle_expression(exp)
    }

    fn interpret_return(&mut self) -> Result<ExecResult, Error> {
        // TODO: make the parser catch invalid returns
        let ret = self.peek().unwrap().expect_return()?.clone();
        return Ok(ExecResult::Returned(
            self.handle_expression(ret.expression)?.expect_value()?,
        ));
    }

    fn handle_expression(&mut self, expression: Expression) -> Result<ExecResult, Error> {
        todo!()
    }

    fn run_function(&mut self, func: Function, args: Vec<Expression>) -> Result<ExecResult, Error> {
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
