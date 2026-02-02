/*
* List type
* */

use crate::interpreter::value::Value;

#[derive(Debug, Clone)]
pub enum List {
    Nil,
    Cons(Cons),
}

#[derive(Debug, Clone)]
pub struct Cons {
    pub head: Box<Value>,
    pub tail: Box<List>,
}

