use crate::{interpreter::scope::Scope, parser::ast::Function};
use std::rc::Rc;

/**
* Closure type
* */

#[derive(Debug, Clone)]
pub struct Closure {
    pub node: Function,
    pub env: Rc<Scope>,
}
