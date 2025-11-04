use lang::{
    error_types::Error,
    file::read_file,
    lexer::tokenize,
    parser::{ast::StatementList, parse},
};

#[test]
fn test_simple_parse() {
    assert!(check(parse_str("i32 hello = 2;")));
    assert!(check(parse_str(
        "i32 function = fn(arg1: i32, arg2: string) { return arg1 + arg2; };"
    )));
}

#[test]
fn test_call() {
    let program = read_file("demos/call.lang").unwrap();
    let tokens = tokenize(program).unwrap();
    let ast = parse(tokens).unwrap();
    // ast.print_ast(0);
}

#[test]
fn test_file_parse() {
    let program = read_file("demos/function.lang").unwrap();
    assert!(check(parse_str(&program)));
    let program = read_file("demos/expression.lang").unwrap();
    assert!(check(parse_str(&program)));
    let program = read_file("demos/print_test.lang").unwrap();
    let tokens = tokenize(program).unwrap();
    let ast = parse(tokens).unwrap();
    // ast.print_ast(0);
    let program = read_file("demos/priority.lang").unwrap();
    let tokens = tokenize(program).unwrap();
    let ast = parse(tokens).unwrap();
    // ast.print_ast(0);
}

#[test]
fn test_file_parse_2() {
}

fn parse_str(program: &str) -> Result<StatementList, Error> {
    let program = program.to_string();
    let tokens = tokenize(program).unwrap();
    parse(tokens)
}

fn check(parse: Result<StatementList, Error>) -> bool {
    match parse {
        Ok(_) => true,
        Err(e) => {
            println!("{:?}", e);
            false
        }
    }
}
