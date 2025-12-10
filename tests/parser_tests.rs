use lang::{
    error_types::Error,
    lexer::tokenize,
    parser::{ast::StatementList, parse},
};

#[test]
fn parse_simple() {
    check(parse_str("i32 digit = 2;"));
}

fn parse_str(program: &str) -> Result<StatementList, Error> {
    let program = program.to_string();
    let tokens = tokenize(program).unwrap();
    parse(tokens)
}

fn check(parse: Result<StatementList, Error>) {
    parse.unwrap_or_else(|e| panic!("parse failed: {:?}", e));
}
