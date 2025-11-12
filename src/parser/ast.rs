/*
* Nonterminal types
* */

use crate::error_types::Error;

#[derive(Debug, Clone)]
pub struct StatementList {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub enum Statement {
    Assignment(Assignment),
    Expression(Expression),
    Return(Return),
}

impl Statement {
    pub fn expect_assignment(&self) -> Result<&Assignment, Error> {
        if let Statement::Assignment(r) = self {
            return Ok(r)
        }
        Err(Error::generic_ust())
    }
    pub fn expect_expression(&self) -> Result<&Expression, Error> {
        if let Statement::Expression(r) = self {
            return Ok(r)
        }
        Err(Error::generic_ust())
    }
    pub fn expect_return(&self) -> Result<&Return, Error> {
        if let Statement::Return(r) = self {
            return Ok(r)
        }
        Err(Error::generic_ust())
    }
}

#[derive(Debug, Clone)]
pub enum Expression {
    LiteralExp(Literal),
    IdentifierExp(Identifier),
    FunctionExp(Function),
    CallExp(Call),
    BinaryExp(BinaryExp),
    IfExp(IfExp),
    ParenExp(Box<Expression>),
}

#[derive(Debug, Clone)]
pub struct Literal {
    pub position: Position,
    pub value: LiteralValue,
}
pub type LiteralValue = crate::lexer::token::Literal;

#[derive(Debug, Clone)]
pub struct Identifier {
    pub position: Position,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct Assignment {
    pub position: Position,
    pub assignment_type: Type,
    pub identifier: String,
    pub expression: Expression,
}

#[derive(Debug, Clone)]
pub struct Return {
    pub position: Position,
    pub expression: Expression,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub position: Position,
    pub params: Vec<Param>,
    pub body: StatementList,
}

#[derive(Debug, Clone)]
pub struct Param {
    // for def
    pub position: Position,
    pub param_type: Type,
    pub identifier: String,
}

#[derive(Debug, Clone)]
pub struct Call {
    pub position: Position,
    // any expression can be called
    pub callee: Box<Expression>,
    pub args: Vec<Argument>,
}
#[derive(Debug, Clone)]
pub struct Argument {
    // for call
    pub position: Position,
    pub value: Expression,
}

#[derive(Debug, Clone)]
pub struct BinaryExp {
    pub position: Position,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
    pub operator: Operator,
}

#[derive(Debug, Clone)]
pub struct IfExp {
    // if is an expression
    pub position: Position,
    pub if_cond: Box<Expression>,
    pub then_branch: Box<Expression>,
    pub else_branch: Option<Box<Expression>>,
}

/*
* Keywords & Operators
* */

#[derive(Debug, Clone)]
pub enum Type {
    I32,
    String,
    Bool,
}

pub type Operator = crate::lexer::token::Operator;

/*
* Metadata
* */

#[derive(Clone, Debug)]
pub struct Position {
    pub start_line: usize,
    pub start_col: usize,
}

impl StatementList {
    pub fn print_ast(&self, indent: usize) {
        for stmt in &self.statements {
            stmt.print_ast(indent);
        }
    }
}
impl Statement {
    pub fn print_ast(&self, indent: usize) {
        let padding = "   ".repeat(indent);
        match self {
            Statement::Return(rst) => {
                println!("{}|_ Return", padding);
                rst.expression.print_ast(indent + 1);
            }
            Statement::Assignment(ast) => {
                println!(
                    "{}|_ Assignment: {:?} {}",
                    padding, ast.assignment_type, ast.identifier
                );
                ast.expression.print_ast(indent + 1);
            }
            Statement::Expression(expr) => {
                println!("{}|_ Expression", padding);
                expr.print_ast(indent + 1);
            }
        }
    }
}
impl Expression {
    pub fn print_ast(&self, indent: usize) {
        let padding = "   ".repeat(indent);
        match self {
            Expression::IdentifierExp(i) => println!("{} |_ Identifier: {}", padding, i.name),
            Expression::LiteralExp(lexp) => println!("{} |_ Literal: {:?}", padding, lexp.value),
            Expression::BinaryExp(bexp) => {
                println!("{} |_ BinaryExp: {:?}", padding, bexp.operator);
                bexp.left.print_ast(indent + 1);
                bexp.right.print_ast(indent + 1);
            }
            Expression::FunctionExp(fexp) => {
                println!("{} |_ Function", padding);
                fexp.body.print_ast(indent + 1);
            }
            Expression::IfExp(iexp) => {
                println!("{}|_ IfExp", padding);
                println!("{}|_ Condition", padding);
                iexp.if_cond.as_ref().print_ast(indent + 1);
                println!("{}|_ Then", padding);
                iexp.then_branch.as_ref().print_ast(indent + 1);
                if let Some(else_branch) = iexp.else_branch.as_ref() {
                    println!("{}|_ Else", padding);
                    else_branch.print_ast(indent + 1);
                }
            }
            Expression::ParenExp(inner) => {
                println!("{}|_ ParenExp", padding);
                inner.print_ast(indent + 1);
            }
            Expression::CallExp(cexp) => {
                println!("{}|_ CallExp", padding);
                println!("{}|_ Callee:", padding);
                cexp.callee.print_ast(indent + 1);
                if cexp.args.is_empty() {
                    println!("{}|_ Args: ()", padding);
                } else {
                    println!("{}|_ Args:", padding);
                    for arg in &cexp.args {
                        arg.value.print_ast(indent + 1);
                    }
                }
            }
        }
    }
}
