use lang::{interpreter::interpret, lexer::tokenize, parser::parse, utils::lang_error::Error};
use std::{fs, path::Path};

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
        let name = entry
            .file_name()
            .to_str()
            .expect("failed to get file name")
            .to_string();
        let program = fs::read_to_string(&path).unwrap();
        let result = test_exec(program);
        println!("\nrunning: {}", name);
        match result {
            Ok(_) => {
                assert!(!should_fail, r#"case failed: {name}"#);
                println!("case passed: {}", name);
            }
            Err(res) => {
                assert!(
                    should_fail,
                    "case failed (unexpected error): {}\n{}",
                    name,
                    res.display()
                );
                println!("case passed: {}", name)
            }
        }
    }
}

fn test_exec(src: String) -> Result<(), Error> {
    let tokens = tokenize(src)?;
    let ast = parse(tokens)?;
    interpret(ast)?;
    Ok(())
}
