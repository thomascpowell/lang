use crate::{interpreter::scope::Scope, parser::ast::Function};
use std::rc::Rc;

/**
* Closure type
* The interpreter representation of a function type
* */

#[derive(Debug, Clone)]
pub struct Closure {
    pub node: Function,
    pub env: Rc<Scope>,
}
