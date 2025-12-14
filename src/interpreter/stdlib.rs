use std::io::{self, BufRead, Read, Write, stdin};

use crate::{
    error_types::{Error, ErrorType},
    interpreter::{exec_result::ExecResult, symbol::Value},
};

/**
* Standard library functions
* */

pub fn std_print(args: Vec<Value>) -> Result<ExecResult, Error> {
    for a in &args {
        print!("{}", a.display());
    }
    Ok(ExecResult::Unit)
}

pub fn std_println(args: Vec<Value>) -> Result<ExecResult, Error> {
    for a in &args {
        print!("{}", a.display());
    }
    println!();
    Ok(ExecResult::Unit)
}

pub fn std_read(_args: Vec<Value>) -> Result<ExecResult, Error> {
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .map_err(|_| Error::new(ErrorType::StdRead, 0, 0, "", None))?;
    Ok(ExecResult::Value(Value::String(buffer)))
}

pub fn std_panic(_args: Vec<Value>) -> Result<ExecResult, Error> {
    panic!("[panic]");
}
