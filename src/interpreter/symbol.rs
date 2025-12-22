use crate::{
    parser::ast::{Position, Type},
};
use crate::interpreter::value::Value;

/**
* Symbol type
* */

#[derive(Clone)]
pub struct Symbol {
    pub pos: Position,
    pub ty: Type,
    pub val: Value,
}
