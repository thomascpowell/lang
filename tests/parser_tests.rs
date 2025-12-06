use std::{fs, path::Path};

use lang::{
    error_types::Error,
    lexer::tokenize,
    parser::{ast::StatementList, parse},
};

#[test]
fn parse_simple() {
    check(parse_str("i32 digit = 2;"));
}

#[test]
fn parse_demos() {
    let demo_dir = Path::new("demos");
    for entry in fs::read_dir(demo_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let program = fs::read_to_string(&path).unwrap();
        let _ = parse_str(&program).map_err(|e| panic!("parse failed for {:?}: {:?}", path, e));
    }
}

fn parse_str(program: &str) -> Result<StatementList, Error> {
    let program = program.to_string();
    let tokens = tokenize(program).unwrap();
    parse(tokens)
}

fn check(parse: Result<StatementList, Error>) {
    parse.unwrap_or_else(|e| panic!("parse failed: {:?}", e));
}
