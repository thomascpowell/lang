use std::{fs, path::Path};

use lang::{interpreter::interpret, lexer::tokenize, parser::parse};

#[test]
fn interpret_simple() {
    test_exec("i32 hello = 1; println(\"from interpret_simple: \", hello);".into());
}

#[test]
fn interpret_demos() {
    let demo_dir = Path::new("demos");
    for entry in fs::read_dir(demo_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let program = fs::read_to_string(&path).unwrap();
        test_exec(program);
    }
}

#[test]
fn invalid_assignment() {
    let src = "i32 x = \"an string\";".to_string();
    let tokens = tokenize(src).expect("failed to tokenize");
    let ast = parse(tokens).expect("failed to parse");
    assert!(interpret(ast).is_err());
}

#[test]
fn invalid_return() {
    // this function returns the wrong type
    let src = "def test = fn(i: i32) -> string { return 1; }; test(1);".to_string();
    let tokens = tokenize(src).expect("failed to tokenize");
    let ast = parse(tokens).expect("failed to parse");
    assert!(interpret(ast).is_err());
}

fn test_exec(src: String) {
    let tokens = tokenize(src).expect("failed to tokenize");
    let ast = parse(tokens).expect("failed to parse");
    interpret(ast).expect("failed to interpret");
}
