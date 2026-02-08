use crate::{interpreter::value::Value, parser::ast::Type, utils::position::Position};

/**
* Symbol type
* */

#[derive(Clone, Debug)]
pub struct Symbol {
    pub pos: Position,
    pub ty: Type,
    pub val: Value,
}
