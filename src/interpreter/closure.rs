use crate::{interpreter::scope::Scope, parser::ast::Function};

/**
* Closure type
* */

pub struct Closure {
    node: Function,
    env: Scope,
}
