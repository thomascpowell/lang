use crate::{
    error_types::Error,
    interpreter::{exec_result::ExecResult, symbol::Value},
};

/**
* Standard library functions
* */

pub fn std_print(args: Vec<Value>) -> Result<ExecResult, Error> {
    for a in &args {
        print!("{:?}", a);
    }
    Ok(ExecResult::Unit)
}

pub fn std_println(args: Vec<Value>) -> Result<ExecResult, Error> {
    for a in &args {
        print!("{:?}", a);
    }
    println!();
    Ok(ExecResult::Unit)
}

pub fn std_panic(_args: Vec<Value>) -> Result<ExecResult, Error> {
    panic!("panic called from script");
}
