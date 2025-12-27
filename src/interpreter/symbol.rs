use crate::interpreter::value::Value;
use crate::parser::ast::Type;
use crate::position::Position;

/**
* Symbol type
* */

#[derive(Clone, Debug)]
pub struct Symbol {
    pub pos: Position,
    pub ty: Type,
    pub val: Value,
}
