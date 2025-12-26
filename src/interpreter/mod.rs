use crate::interpreter::{scope::get_stdlib_scope, value::Value};
use std::{ops, rc::Rc};

use crate::{
    error_types::{Error, ErrorType},
    interpreter::{exec_result::ExecResult, frame::Frame, scope::Scope, symbol::*},
    parser::ast::*,
};

pub mod closure;
pub mod exec_result;
pub mod frame;
pub mod scope;
pub mod stdlib;
pub mod symbol;
pub mod value;

/**
* Interpreter
* */

pub fn interpret(ast: StatementList) -> Result<(), Error> {
    let mut interpreter = Interpreter::new(ast);
    interpreter.run_program()?;
    Ok(())
}

pub struct Interpreter {
    pub frames: Vec<Frame>,
    pub scope: Rc<Scope>,
}

impl Interpreter {
    pub fn new(ast: StatementList) -> Self {
        Interpreter {
            frames: vec![Frame::new(ast)],
            scope: get_stdlib_scope(),
        }
    }

    // runs frame 0 (entire program)
    pub fn run_program(&mut self) -> Result<(), Error> {
        let result = self.run_frame()?;
        // frame 0 cannot have a return value
        if let ExecResult::Returned(_) = result {
            return Err(Error::generic_message(
                ErrorType::InvalidReturnLocation,
                "return must be inside a function".to_string(),
            ));
        }
        Ok(())
    }

    pub fn run_frame(&mut self) -> Result<ExecResult, Error> {
        loop {
            // get the frame off the stack
            let mut frame = match self.frames.pop() {
                Some(f) => f,
                None => return Ok(ExecResult::Unit),
            };
            // case: implicit return
            if frame.done() {
                return Ok(ExecResult::Unit);
            }
            // execute a statement
            let stmt = frame.peek().expect("statement");
            let result = self.exec(stmt)?;
            match result {
                // case: return
                ExecResult::Returned(v) => {
                    return Ok(ExecResult::Returned(v));
                }
                // case: continue
                // (put the frame back)
                _ => {
                    frame.advance();
                    self.frames.push(frame);
                }
            }
        }
    }

    fn exec(&mut self, stmt: &Statement) -> Result<ExecResult, Error> {
        return match stmt {
            Statement::Assignment(a) => self.interpret_assignment(a),
            Statement::Expression(e) => self.handle_expression(&e),
            Statement::Return(r) => self.interpret_return(r),
        };
    }

    fn interpret_assignment(&mut self, a: &Assignment) -> Result<ExecResult, Error> {
        let symbol_value = self.handle_expression(&a.expression)?.expect_value()?;
        let symbol = Symbol {
            pos: a.position.clone(),
            ty: a.assignment_type.clone(),
            val: symbol_value,
        };
        // make sure symbol matches
        let ty = symbol.val.get_type();
        if symbol.ty != ty {
            return Err(Error::new(
                ErrorType::TypeMismatch,
                symbol.pos,
                format!("{:?}", ty),
                Some("invalid assignment type"),
            ));
        }
        // push to the scope stack
        self.scope = self.scope.extend(a.identifier.clone(), symbol);
        // self.set_symbol(&a.identifier, symbol)?;
        Ok(ExecResult::Unit)
    }

    fn interpret_return(&mut self, r: &Return) -> Result<ExecResult, Error> {
        Ok(ExecResult::Returned(
            self.handle_expression(&r.expression)?.expect_value()?,
        ))
    }

    fn handle_expression(&mut self, expression: &Expression) -> Result<ExecResult, Error> {
        match expression {
            Expression::IdentifierExp(exp) => Ok(ExecResult::Value(self.handle_identifer(exp)?)),
            Expression::FunctionExp(exp) => Ok(ExecResult::Value(Value::Function(exp.clone()))),
            Expression::CallExp(exp) => self.handle_call(exp.clone()),
            Expression::LiteralExp(exp) => self.handle_literal(exp.clone()),
            Expression::BinaryExp(exp) => self.handle_binary(exp.clone()),
            Expression::IfExp(exp) => self.handle_if(exp.clone()),
            Expression::ParenExp(exp) => self.handle_paren(exp.clone()),
        }
    }

    fn handle_binary(&mut self, exp: BinaryExp) -> Result<ExecResult, Error> {
        let left_val = self.handle_expression(&exp.left)?.expect_value()?;
        let right_val = self.handle_expression(&exp.right)?.expect_value()?;
        let left_type = left_val.get_type();
        let right_type = right_val.get_type();
        let position = exp.position;

        let arithmetic_operand_error = Error::new(
            ErrorType::InvalidOperand,
            position,
            format!("{:?} {:?} {:?}", left_type, exp.operator, right_type),
            Some("operand types must match"),
        );

        let res: Value = match exp.operator {
            // modulo operator:
            // - operands: i32
            // - returns: i32
            Operator::Mod => Value::Int(left_val.expect_int()? % right_val.expect_int()?),
            // arithmetic operators: 
            // - operands: numeric types, must match
            // - returns: same type
            Operator::Mul => (left_val * right_val).ok_or_else(|| arithmetic_operand_error)?,
            Operator::Div => (left_val / right_val).ok_or_else(|| arithmetic_operand_error)?,
            Operator::Add => (left_val + right_val).ok_or_else(|| arithmetic_operand_error)?,
            Operator::Sub => (left_val - right_val).ok_or_else(|| arithmetic_operand_error)?,
            // comparison operators
            // - operands: numeric types
            // - returns: bool
            Operator::Le => Value::Bool(left_val.expect_numeric()? <= right_val.expect_numeric()?),
            Operator::Ge => Value::Bool(left_val.expect_numeric()? >= right_val.expect_numeric()?),
            Operator::Lt => Value::Bool(left_val.expect_numeric()? < right_val.expect_numeric()?),
            Operator::Gt => Value::Bool(left_val.expect_numeric()? > right_val.expect_numeric()?),
            Operator::Eq => Value::Bool(left_val.expect_numeric()? == right_val.expect_numeric()?),
            Operator::Ne => Value::Bool(left_val.expect_numeric()? != right_val.expect_numeric()?),
            // boolean operators
            // - operands: bool
            // - returns: bool
            Operator::And => Value::Bool(left_val.expect_bool()? && right_val.expect_bool()?),
            Operator::Or => Value::Bool(left_val.expect_bool()? || right_val.expect_bool()?),
            _ => unreachable!(),
        };
        Ok(ExecResult::Value(res))
    }

    fn handle_if(&mut self, exp: IfExp) -> Result<ExecResult, Error> {
        let cond = self
            .handle_expression(&exp.if_cond)?
            .expect_value()?
            .expect_bool()?;
        if cond {
            return self.handle_expression(&exp.then_branch);
        }
        match exp.else_branch {
            Some(exp) => self.handle_expression(&exp),
            None => Ok(ExecResult::Unit),
        }
    }

    fn handle_paren(&mut self, exp: Box<Expression>) -> Result<ExecResult, Error> {
        self.handle_expression(&exp)
    }

    fn handle_literal(&mut self, lit: Literal) -> Result<ExecResult, Error> {
        Ok(match lit.value {
            LiteralValue::Int(x) => ExecResult::Value(Value::Int(x)),
            LiteralValue::Float(x) => ExecResult::Value(Value::Float(x)),
            LiteralValue::String(x) => ExecResult::Value(Value::String(x)),
            LiteralValue::Bool(x) => ExecResult::Value(Value::Bool(x)),
        })
    }

    fn handle_call(&mut self, call: Call) -> Result<ExecResult, Error> {
        let callee = call.callee;
        match *callee {
            Expression::FunctionExp(function) => self.run_function(function, call.args),
            Expression::IdentifierExp(identifier) => {
                let symbol = self
                    .scope
                    .get_symbol(&identifier.name, identifier.position.clone())?
                    .val
                    .clone();
                match symbol {
                    // if the corresponding value is a function, run it
                    Value::Function(x) => self.run_function(x, call.args),
                    // case of stdlib call
                    Value::NativeFunction(f) => {
                        // evaluate arguments first
                        let mut arg_values = Vec::new();
                        for arg in call.args {
                            arg_values.push(self.handle_expression(&arg.value)?.expect_value()?);
                        }
                        f(arg_values)
                    }
                    // otherwise, return unit type
                    _ => Ok(ExecResult::Unit),
                }
            }
            // otherwise, return unit type (result of calling anything else)
            _ => Ok(ExecResult::Unit),
        }
    }

    fn handle_identifer(&mut self, identifier: &Identifier) -> Result<Value, Error> {
        Ok(self
            .scope
            .get_symbol(&identifier.name, identifier.position.clone())?
            .val
            .clone())
    }

    fn run_function(&mut self, func: Function, args: Vec<Argument>) -> Result<ExecResult, Error> {
        let num_params = func.params.len();
        let num_args = args.len();
        let position = func.position;
        // ensure correct number of arguments are passed
        if num_params != num_args {
            return Err(Error::new(
                ErrorType::InvalidParams,
                position,
                format!("{:?}", args.len()),
                Some("incorrect number of arguments"),
            ));
        }
        let mut binds: Vec<(String, Symbol)> = Vec::new();
        let mut evaluated_args: Vec<Symbol> = Vec::new();
        for arg in &args {
            let value = self
                .handle_expression(&arg.value)?
                .expect_value()?
                .into_symbol(position.clone());
            evaluated_args.push(value);
        }
        for i in 0..num_args {
            let param = &func.params[i];
            let arg_symbol = &evaluated_args[i];
            if arg_symbol.ty != param.param_type {
                return Err(Error::new(
                    ErrorType::TypeMismatch,
                    position,
                    "type mismatch",
                    Some("check function call"),
                ));
            }
            binds.push((param.identifier.clone(), arg_symbol.clone()));
        }
        self.scope = self.scope.extend_many(binds);
        // push a new frame
        self.frames.push(Frame::new(func.body.clone()));
        let result = self.run_frame()?;
        self.pop_scope();
        let res = match result {
            ExecResult::Returned(v) => ExecResult::Value(v),
            _ => unreachable!(),
        };
        let function_returned_type = res.expect_value()?.get_type();
        if function_returned_type != func.returns {
            return Err(Error::new(
                ErrorType::TypeMismatch,
                position,
                format!("{:?}", function_returned_type),
                Some("function returns wrong type"),
            ));
        }
        Ok(res)
    }

    /*
     * Utility functions
     * */

    fn pop_scope(&mut self) {
        let parent = self.scope.parent.clone().expect("expected scope");
        self.scope = parent;
    }
}
