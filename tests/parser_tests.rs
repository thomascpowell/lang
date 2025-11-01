use lang::{
    error_types::Error,
    lexer::tokenize,
    parser::{ast::StatementList, parse},
};

#[test]
fn test_simple_parse() {
    assert!(parse_str("i32 hello = 2;").is_ok());
    assert!(
        parse_str("i32 function = fn(arg1: i32, arg2: string) { return arg1 + arg2; };").is_ok()
    );
    assert!(parse_str("i32 hello = 2;").is_ok());
    assert!(parse_str("i32").is_err());
}

fn parse_str(program: &str) -> Result<StatementList, Error> {
    let program = program.to_string();
    println!("program: {}", program);
    let tokens = tokenize(program).unwrap();
    println!("tokens: {}", tokens.len());
    parse(tokens)
}
