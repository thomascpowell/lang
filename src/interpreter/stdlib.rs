use std::io::{self, Write};

use crate::{
    interpreter::{exec_result::ExecResult, list::List, value::Value},
    lang_error::{Error, ErrorType},
    position::Position,
};

/**
* Standard library functions
* */

const POSITION: Position = Position { line: 0, col: 0 };

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
        .map_err(|_| Error::new(ErrorType::StdRead, POSITION, "", None))?;
    Ok(ExecResult::Value(Value::String(buffer)))
}

pub fn std_floor(args: Vec<Value>) -> Result<ExecResult, Error> {
    Ok(ExecResult::Value(Value::Int(
        get_arg(&args, 0)?.expect_float()? as i32,
    )))
}

pub fn std_new_list(_args: Vec<Value>) -> Result<ExecResult, Error> {
    Ok(ExecResult::Value(Value::List(List::Nil)))
}

pub fn std_head(args: Vec<Value>) -> Result<ExecResult, Error> {
    let list = get_arg(&args, 0)?.expect_list()?.clone();
    match list {
        List::Nil => {
            return Err(Error::new(
                ErrorType::EmptyList,
                POSITION,
                "expected head to exist",
                None,
            ));
        }
        List::Cons(c) => Ok(ExecResult::Value(c.head.as_ref().clone())),
    }
}

pub fn std_tail(args: Vec<Value>) -> Result<ExecResult, Error> {
    let list = get_arg(&args, 0)?.expect_list()?.clone();
    match list {
        List::Nil => {
            return Err(Error::new(
                ErrorType::EmptyList,
                POSITION,
                "expected head to exist",
                None,
            ));
        }
        List::Cons(c) => Ok(ExecResult::Value(Value::List(c.tail.as_ref().clone()))),
    }
}

pub fn std_assert(args: Vec<Value>) -> Result<ExecResult, Error> {
    let cond = get_arg(&args, 0)?.expect_bool()?;
    let optional_msg = args
        .get(1)
        .and_then(|r| return Some(r.expect_string().unwrap()));
    if !cond {
        return Err(Error::new(
            ErrorType::StdAssertionFailure,
            POSITION,
            "assertion failure",
            optional_msg.as_deref(),
        ));
    };
    Ok(ExecResult::Unit)
}

pub fn std_panic(_args: Vec<Value>) -> Result<ExecResult, Error> {
    panic!("[panic]");
}

/**
* Helper functions
* */

fn get_arg<'a>(args: &'a Vec<Value>, index: usize) -> Result<&'a Value, Error> {
    args.get(index).ok_or(Error::new(
        ErrorType::StdMissingArgs,
        POSITION,
        "missing or incorrect argument to stdlib function",
        None,
    ))
}
