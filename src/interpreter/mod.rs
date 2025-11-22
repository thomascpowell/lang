use crate::{error_types::Error, interpreter::symbol::*, parser::ast::*};
pub mod symbol;

/**
* Interpreter
* Uses dynamic scoping (unfortunately)
*
* */

// this matches the general structure of the other components
pub fn interpret(ast: StatementList) -> Result<(), Error> {
    let mut interpreter = Interpreter::new(ast);
    while interpreter.has_next() {
        match interpreter.interpret_statement()? {
            ExecResult::Returned(_) => {
                return Err(Error::generic_message(
                    crate::error_types::ErrorType::InvalidReturnLocation,
                    "return must be in a function".to_string(),
                ));
            }
            _ => interpreter.advance(),
        }
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
        match expression {
            Expression::IdentifierExp(exp) => Ok(ExecResult::Value(self.handle_identifer(exp)?)),
            Expression::FunctionExp(exp) => Ok(ExecResult::Value(Value::Function(exp))),
            Expression::CallExp(exp) => self.handle_call(exp),
            Expression::LiteralExp(exp) => self.handle_literal(exp),

            // TODO
            Expression::BinaryExp(_) => Ok(ExecResult::Unit),
            Expression::IfExp(_) => Ok(ExecResult::Unit),
            Expression::ParenExp(_) => Ok(ExecResult::Unit),
        }
    }

    fn handle_binary(&mut self, exp: BinaryExp) -> Result<ExecResult, Error> {
        todo!();
    }

    fn handle_if(&mut self, exp: IfExp) -> Result<ExecResult, Error> {
        todo!();
    }

    fn handle_paren(&mut self, exp: Box<Expression>) -> Result<ExecResult, Error> {
        todo!();
    }

    fn handle_literal(&mut self, lit: Literal) -> Result<ExecResult, Error> {
        Ok(match lit.value {
            LiteralValue::Int(x) => ExecResult::Value(Value::Int(x)),
            LiteralValue::String(x) => ExecResult::Value(Value::String(x)),
            LiteralValue::Bool(x) => ExecResult::Value(Value::Bool(x)),
        })
    }

    fn handle_call(&mut self, call: Call) -> Result<ExecResult, Error> {
        let callee = call.callee;
        match *callee {
            Expression::FunctionExp(function) => self.run_function(function, call.args),
            Expression::IdentifierExp(identifier) => {
                match self.scopes.get_symbol(&identifier.name)?.val.clone() {
                    // if the corresponding value is a function, run it
                    Value::Function(x) => self.run_function(x, call.args),
                    // otherwise, return unit type
                    _ => Ok(ExecResult::Unit),
                }
            }
            // otherwise, return unit type (result of calling anything else)
            _ => Ok(ExecResult::Unit),
        }
    }

    fn handle_identifer(&mut self, identifier: Identifier) -> Result<Value, Error> {
        Ok(self.scopes.get_symbol(&identifier.name)?.val.clone())
    }

    fn run_function(&mut self, func: Function, args: Vec<Argument>) -> Result<ExecResult, Error> {
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
