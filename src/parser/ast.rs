use crate::{error_types::Error, position::Position};

/*
* Nonterminal types
* */

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
    pub fn get_position(&self) -> &Position {
        return match self {
            Statement::Return(x) => &x.expression.get_position(),
            Statement::Assignment(x) => &x.position,
            Statement::Expression(x) => x.get_position(),
        };
    }

    pub fn expect_assignment(&self) -> Result<&Assignment, Error> {
        if let Statement::Assignment(r) = self {
            return Ok(r);
        }
        Err(Error::generic_ust(self))
    }

    pub fn expect_expression(&self) -> Result<&Expression, Error> {
        if let Statement::Expression(r) = self {
            return Ok(r);
        }
        Err(Error::generic_ust(self))
    }

    pub fn expect_return(&self) -> Result<&Return, Error> {
        if let Statement::Return(r) = self {
            return Ok(r);
        }
        Err(Error::generic_ust(self))
    }
}

#[derive(Debug, Clone)]
pub enum Expression {
    LiteralExp(Literal),
    IdentifierExp(Identifier),
    FunctionExp(Function),
    CallExp(Call),
    ParenExp(Box<Expression>),
    BinaryExp(BinaryExp),
    IfExp(IfExp),
}

impl Expression {
    pub fn get_position(&self) -> &Position {
        match self {
            Expression::LiteralExp(x) => &x.position,
            Expression::IdentifierExp(x) => &x.position,
            Expression::FunctionExp(x) => &x.position,
            Expression::CallExp(x) => &x.position,
            Expression::BinaryExp(x) => &x.position,
            Expression::IfExp(x) => &x.position,
            Expression::ParenExp(x) => &x.get_position(),
        }
    }
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
    pub expression: Expression,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub position: Position,
    pub params: Vec<Param>,
    pub returns: Type,
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

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    I32,
    F32,
    String,
    Bool,
    Function,
    Unit,
}

pub type Operator = crate::lexer::token::Operator;

/**
* Printing AST
* */

fn get_padding(indent: usize) -> String {
    "|  ".repeat(indent)
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
        let padding = get_padding(indent);
        match self {
            Statement::Return(rst) => {
                println!("{}Return", padding);
                rst.expression.print_ast(indent + 1);
            }
            Statement::Assignment(ast) => {
                println!(
                    "{}Assignment: {:?} {}",
                    padding, ast.assignment_type, ast.identifier
                );
                ast.expression.print_ast(indent + 1);
            }
            Statement::Expression(expr) => {
                println!("{}Expression", padding);
                expr.print_ast(indent + 1);
            }
        }
    }
}
impl Expression {
    pub fn print_ast(&self, indent: usize) {
        let padding = "|  ".repeat(indent);
        match self {
            Expression::IdentifierExp(i) => println!("{}Identifier: {}", padding, i.name),
            Expression::LiteralExp(lexp) => println!("{}Literal: {:?}", padding, lexp.value),
            Expression::BinaryExp(bexp) => {
                println!("{}BinaryExp: {:?}", padding, bexp.operator);
                bexp.left.print_ast(indent + 1);
                bexp.right.print_ast(indent + 1);
            }
            Expression::FunctionExp(fexp) => {
                println!("{}Function", padding);
                fexp.body.print_ast(indent + 1);
            }
            Expression::IfExp(iexp) => {
                println!("{}IfExp", padding);
                println!("{}Condition", padding);
                iexp.if_cond.as_ref().print_ast(indent + 1);
                println!("{}Then", padding);
                iexp.then_branch.as_ref().print_ast(indent + 1);
                if let Some(else_branch) = iexp.else_branch.as_ref() {
                    println!("{}Else", padding);
                    else_branch.print_ast(indent + 1);
                }
            }
            Expression::ParenExp(inner) => {
                println!("{}ParenExp", padding);
                inner.print_ast(indent + 1);
            }
            Expression::CallExp(cexp) => {
                println!("{}CallExp", padding);
                println!("{}Callee:", padding);
                cexp.callee.print_ast(indent + 1);
                if cexp.args.is_empty() {
                    println!("{}Args: ()", padding);
                } else {
                    println!("{}Args:", padding);
                    for arg in &cexp.args {
                        arg.value.print_ast(indent + 1);
                    }
                }
            }
        }
    }
}
