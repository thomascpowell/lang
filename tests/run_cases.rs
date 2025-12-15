use std::{fs, path::Path};

use lang::{error_types::Error, interpreter::interpret, lexer::tokenize, parser::parse};

#[test]
fn run_positive_cases() {
    run_cases(Path::new("./tests/cases/positive"), false);
}

#[test]
fn run_negative_cases() {
    run_cases(Path::new("./tests/cases/negative"), true);
}

fn run_cases(dir: &Path, should_fail: bool) {
    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let name = entry.file_name().to_str().expect("failed to get file name").to_string();
        let program = fs::read_to_string(&path).unwrap();
        if should_fail {
            assert!(test_exec(program).is_err(), r#"{name} should fail"#);
        } else {
            assert!(test_exec(program).is_ok(), r#"{name} should pass"#);
        }
    }
}

fn test_exec(src: String) -> Result<(), Error> {
    let tokens = tokenize(src)?;
    let ast = parse(tokens)?;
    interpret(ast)?;
    Ok(())
}
