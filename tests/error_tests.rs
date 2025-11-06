use lang::{lexer::tokenize, parser::parse};

// this is mostly to confirm that errors _look_ right
// not so much that they are used appropriately
#[test]
fn test_parser_error() {
    let res = parse(tokenize("invalid source".to_string()).unwrap());
    assert!(res.is_err());
    println!("{}", res.unwrap_err().display())
}

