use crate::{interpreter::value::Value, utils::lang_error::Error};

/**
* ExecResult type
* */

#[derive(Debug)]
pub enum ExecResult {
    Value(Value),
    Returned(Value),
}

impl ExecResult {
    pub fn expect_value(&self) -> Result<Value, Error> {
        if let ExecResult::Value(v) = self {
            return Ok(v.clone());
        }
        Err(Error::generic_uer(self, "ExecResult::Value"))
    }
    pub fn expect_returned(&self) -> Result<Value, Error> {
        if let ExecResult::Returned(r) = self {
            return Ok(r.clone());
        }
        Err(Error::generic_uer(self, "ExecResult::Returned"))
    }
}
