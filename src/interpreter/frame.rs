use crate::parser::ast::{Statement, StatementList};

/**
* Frame type
* */

#[derive(Clone)]
pub struct Frame {
    pub pos: usize,
    pub ast: StatementList,
}

impl Frame {
    pub fn new(ast: StatementList) -> Self {
        Frame { ast: ast, pos: 0 }
    }

    pub fn peek(&self) -> Option<&Statement> {
        self.ast.statements.get(self.pos)
    }

    pub fn advance(&mut self) {
        self.pos += 1
    }

    pub fn done(&self) -> bool {
        self.pos >= self.ast.statements.len()
    }
}
