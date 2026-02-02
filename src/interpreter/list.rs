/*
* List type
* */

use crate::interpreter::value::Value;

#[derive(Debug, Clone)]
pub enum List {
    Nil,
    Cons(Cons),
}

impl List {
    pub fn display(&self, initial: bool) -> String {
        let open = if initial { "[" } else { "" };
        match self {
            Self::Nil => format!("{} ]", open),
            Self::Cons(c) => format!("{} {}{}", open, c.head.display(), c.tail.display(false)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Cons {
    pub head: Box<Value>,
    pub tail: Box<List>,
}
