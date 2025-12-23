use lang::{lexer::tokenize, parser::parse};

#[test]
fn test_parser_error() {
    let res = parse(tokenize("invalid source example;;".to_string()).unwrap());
    assert!(res.is_err());
    println!("{}", res.unwrap_err().display())
}

