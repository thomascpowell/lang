use crate::{error_types::Error, interpreter::symbol::Value};

pub enum ExecResult {
    Unit,            // statement produces nothing
    Value(Value),    // expression result
    Returned(Value), // function return
}

impl ExecResult {
    pub fn expect_unit(&self) -> Result<(), Error> {
        if matches!(self, ExecResult::Unit) {
            return Ok(());
        }
        Err(Error::generic_uer())
    }
    pub fn expect_value(&self) -> Result<Value, Error> {
        if let ExecResult::Value(v) = self {
            return Ok(v.clone());
        }
        Err(Error::generic_uer())
    }
    pub fn expect_returned(&self) -> Result<Value, Error> {
        if let ExecResult::Returned(r) = self {
            return Ok(r.clone());
        }
        Err(Error::generic_uer())
    }
}
