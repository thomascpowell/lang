use lang::{error_types::Error, interpreter::interpret, lexer::tokenize, parser::parse};
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
        println!("\n\nrunning {}", name);
        let program = fs::read_to_string(&path).unwrap();
        let result = test_exec(program);
        match result {
            Ok(_) => {
                assert!(!should_fail, r#"{name} should fail"#)
            }
            Err(res) => {
                assert!(should_fail, r#"{name} should succeed"#);
                println!("\n\n{} has failed (expected):\n{}", name, res.display())
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
