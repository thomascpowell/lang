use crate::interpreter::{
    closure::Closure,
    list::{Cons, List},
    scope::*,
    value::Value,
};
use std::{cell::RefCell, rc::Rc};

use crate::{
    interpreter::{exec_result::ExecResult, frame::Frame, scope::Scope, symbol::*},
    parser::ast::*,
    utils::lang_error::{Error, ErrorType},
};

pub mod closure;
pub mod exec_result;
pub mod frame;
pub mod list;
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
        let initial_scope = self.scope.clone();
        loop {
            // get the frame off the stack
            let mut frame = match self.frames.pop() {
                Some(f) => f,
                None => return Ok(ExecResult::Unit),
            };
            // case: EOF
            if frame.done() {
                assert!(self.frames.is_empty());
                return Ok(ExecResult::Unit);
            }
            // execute a statement
            let stmt = frame.peek().expect("statement");
            let result = self.exec(stmt)?;
            match result {
                // case: return
                ExecResult::Returned(v) => {
                    // restore the scope if the function returns
                    self.scope = initial_scope;
                    return Ok(ExecResult::Returned(v));
                }
                // case: continue (put the frame back)
                _ => {
                    frame.advance();
                    self.frames.push(frame);
                }
            }
        }
    }

    fn exec(&mut self, stmt: &Statement) -> Result<ExecResult, Error> {
        match stmt {
            Statement::Assignment(a) => self.interpret_assignment(a),
            Statement::Expression(e) => self.handle_expression(&e),
            Statement::Return(r) => self.interpret_return(r),
        }
    }

    fn interpret_assignment(&mut self, a: &Assignment) -> Result<ExecResult, Error> {
        if a.assignment_type == Type::Function {
            return self.handle_closure(a);
        }
        let rhs = self.handle_expression(&a.expression)?.expect_value()?;
        let symbol = Symbol {
            pos: a.position.clone(),
            ty: a.assignment_type.clone(),
            val: rhs,
        };
        // validate types
        if a.assignment_type != symbol.val.get_type() {
            return Err(Error::new(
                ErrorType::TypeMismatch,
                a.position.clone(),
                format!("{:?}", symbol.ty),
                Some("invalid assignment type"),
            ));
        }
        // bind
        let name = a.identifier.clone();
        let cell = Rc::new(RefCell::new(symbol));
        self.scope = self.scope.extend(name, cell);
        Ok(ExecResult::Unit)
    }

    fn handle_closure(&mut self, a: &Assignment) -> Result<ExecResult, Error> {
        let name = a.identifier.clone();
        let symbol = Symbol {
            pos: a.position.clone(),
            ty: Type::Function,
            val: Value::Uninitialized,
        };
        // bind name to cell
        let cell = Rc::new(RefCell::new(symbol));
        self.scope = self.scope.extend(name, cell.clone());
        // evaluate rhs
        let rhs = self.handle_expression(&a.expression)?.expect_value()?;
        // update/patch the cell (enables recursion)
        *cell.borrow_mut() = Symbol {
            pos: a.position.clone(),
            ty: Type::Function,
            val: rhs,
        };
        Ok(ExecResult::Unit)
    }

    fn interpret_return(&mut self, r: &Return) -> Result<ExecResult, Error> {
        Ok(ExecResult::Returned(
            self.handle_expression(&r.expression)?.expect_value()?,
        ))
    }

    fn handle_expression(&mut self, expression: &Expression) -> Result<ExecResult, Error> {
        match expression {
            Expression::ConsExp(exp) => self.handle_cons(exp.clone()),
            Expression::ListExp(exp) => self.handle_list_exp(exp.clone()),
            Expression::IdentifierExp(exp) => Ok(ExecResult::Value(self.handle_identifer(exp)?)),
            Expression::CallExp(exp) => self.handle_call(exp.clone()),
            Expression::LiteralExp(exp) => self.handle_literal(exp.clone()),
            Expression::BinaryExp(exp) => self.handle_binary(exp.clone()),
            Expression::IfExp(exp) => self.handle_if(exp.clone()),
            Expression::ParenExp(exp) => self.handle_paren(exp.clone()),
            Expression::FunctionExp(exp) => Ok(ExecResult::Value(Value::Function(Closure {
                node: exp.clone(),
                env: Rc::clone(&self.scope),
            }))),
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
            // modulo operator: (operands: i32; returns: i32)
            Operator::Mod => Value::Int(left_val.expect_int()? % right_val.expect_int()?),
            // arithmetic operators: (operands: numeric types, must match; returns: same type)
            Operator::Mul => (left_val * right_val).ok_or_else(|| arithmetic_operand_error)?,
            Operator::Div => (left_val / right_val).ok_or_else(|| arithmetic_operand_error)?,
            Operator::Add => (left_val + right_val).ok_or_else(|| arithmetic_operand_error)?,
            Operator::Sub => (left_val - right_val).ok_or_else(|| arithmetic_operand_error)?,
            // comparison operators: (operands: numeric types; returns: bool)
            Operator::Le => Value::Bool(left_val.expect_numeric()? <= right_val.expect_numeric()?),
            Operator::Ge => Value::Bool(left_val.expect_numeric()? >= right_val.expect_numeric()?),
            Operator::Lt => Value::Bool(left_val.expect_numeric()? < right_val.expect_numeric()?),
            Operator::Gt => Value::Bool(left_val.expect_numeric()? > right_val.expect_numeric()?),
            // TODO: make these also valid for boolean
            Operator::Eq => Value::Bool(left_val.expect_numeric()? == right_val.expect_numeric()?),
            Operator::Ne => Value::Bool(left_val.expect_numeric()? != right_val.expect_numeric()?),
            // boolean operators (operands: bool; returns: bool)
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
        let then_branch = exp.then_branch.as_ref();
        let else_branch = exp.else_branch.as_ref();
        if cond {
            self.exec(then_branch)
        } else {
            else_branch.map_or(Ok(ExecResult::Unit), |exp| self.exec(exp))
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
            LiteralValue::Unit => ExecResult::Value(Value::Unit),
        })
    }

    fn handle_list_exp(&mut self, lexp: ListExp) -> Result<ExecResult, Error> {
        let mut res = List::Nil;
        for item in lexp.items.iter().rev() {
            let val = self.handle_expression(&item)?.expect_value()?;
            let length = res.length() + 1;
            res = List::Cons(Cons {
                head: Box::new(val),
                tail: Box::new(res),
                length: length,
            });
        }
        Ok(ExecResult::Value(Value::List(res)))
    }

    fn handle_call(&mut self, call: Call) -> Result<ExecResult, Error> {
        let callee = self
            .handle_expression(call.callee.as_ref())?
            .expect_value()?;
        match callee.get_type() {
            Type::Function => self.handle_call_function(callee, call),
            _ => Ok(ExecResult::Unit),
        }
    }

    fn handle_call_function(&mut self, value: Value, call: Call) -> Result<ExecResult, Error> {
        match value {
            // if the corresponding value is a function, run the closure
            Value::Function(x) => self.run_closure(x, call.args),
            // case of stdlib call
            Value::NativeFunction(f) => {
                // evaluate arguments first
                let mut arg_values = Vec::new();
                for arg in call.args {
                    arg_values.push(self.handle_expression(&arg.value)?.expect_value()?);
                }
                f(arg_values)
            }
            // otherwise, return unit type (calling any other value, e.g. 7())
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

    fn run_closure(&mut self, closure: Closure, args: Vec<Argument>) -> Result<ExecResult, Error> {
        let num_params = closure.node.params.len();
        let num_args = args.len();
        let func = closure.node;
        let position = func.position;
        let closure_scope = closure.env;
        // ensure correct number of arguments are passed
        if num_params != num_args {
            return Err(Error::new(
                ErrorType::InvalidParams,
                position,
                format!("found: {:?}, expected {:?}", num_args, num_params),
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
        let old_scope = self.scope.clone();
        // switch to the closure scope
        // closure scope = enviroment it was defined + args
        self.scope = closure_scope.extend_many(binds);
        // push a new frame
        self.frames.push(Frame::new(func.body.clone()));
        let result = self.run_frame()?;
        self.scope = old_scope;
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

    fn handle_cons(&mut self, exp: ConsExp) -> Result<ExecResult, Error> {
        let head = self.handle_expression(exp.head.as_ref())?.expect_value()?;
        let tail = self
            .handle_expression(exp.tail.as_ref())?
            .expect_value()?
            .expect_list()?;
        let length = tail.length() + 1;
        let res = List::Cons(Cons {
            head: Box::new(head),
            tail: Box::new(tail),
            length: length,
        });
        Ok(ExecResult::Value(Value::List(res)))
    }
}
