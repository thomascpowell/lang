use std::io::{self, Write};

use crate::{
    error_types::{Error, ErrorType},
    interpreter::{exec_result::ExecResult, value::Value},
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

pub fn std_floor(args: Vec<Value>) -> Result<ExecResult, Error> {
    Ok(ExecResult::Value(Value::Int(
        args.get(0)
            .ok_or(Error::new(
                ErrorType::StdMissingArgs,
                0,
                0,
                "missing argument to stdlib function",
                None,
            ))?
            .expect_float()? as i32,
    )))
}

pub fn std_assert(args: Vec<Value>) -> Result<ExecResult, Error> {
    let optional_msg = args
        .get(1)
        .and_then(|r| return Some(r.expect_string().unwrap()));

    let cond = args
        .get(0)
        .ok_or(Error::new(
            ErrorType::StdMissingArgs,
            0,
            0,
            "missing argument to stdlib function",
            None,
        ))?
        .expect_bool()?;

    if !cond {
        return Err(Error::new(
            ErrorType::StdAssertionFailure,
            0,
            0,
            "assertion failure",
            optional_msg.as_deref(),
        ));
    };

    Ok(ExecResult::Unit)
}

pub fn std_panic(_args: Vec<Value>) -> Result<ExecResult, Error> {
    panic!("[panic]");
}
