use lang::lexer::token::*;
use lang::lexer::tokenize;

#[test]
fn test_tokenizer() {
    let program = r#" 
    fn main(n: i32) -> i32 {
        // some comment
        string x = "hello";
        i32 y = 4 / n;
        return 0;
    } // some comment
    "#
    .to_string();

    let expected = vec![
        TokenKind::Keyword(Keyword::Fn),
        TokenKind::Identifier("main".to_string()),
        TokenKind::Separator(Separator::LParen),
        TokenKind::Identifier("n".to_string()),
        TokenKind::Separator(Separator::Colon),
        TokenKind::Keyword(Keyword::I32),
        TokenKind::Separator(Separator::RParen),
        TokenKind::Separator(Separator::Arrow),
        TokenKind::Keyword(Keyword::I32),
        TokenKind::Separator(Separator::LBrace),
        // TokenKind::Comment("some comment".to_string()),
        TokenKind::Keyword(Keyword::String),
        TokenKind::Identifier("x".to_string()),
        TokenKind::Operator(Operator::Assign),
        TokenKind::Literal(Literal::String("hello".to_string())),
        TokenKind::Separator(Separator::Semicolon),
        TokenKind::Keyword(Keyword::I32),
        TokenKind::Identifier("y".to_string()),
        TokenKind::Operator(Operator::Assign),
        TokenKind::Literal(Literal::Int(4)),
        TokenKind::Operator(Operator::Div),
        TokenKind::Identifier("n".to_string()),
        TokenKind::Separator(Separator::Semicolon),
        TokenKind::Keyword(Keyword::Return),
        TokenKind::Literal(Literal::Int(0)),
        TokenKind::Separator(Separator::Semicolon),
        TokenKind::Separator(Separator::RBrace),
    ];
    compare_output(program, expected, false);
}

#[test]
fn test_operators_separators() {
    let program = "%{}(())+ -/ * ==!= <;; > <=>= && || ! =: - ->".to_string();
    let expected = vec![
        TokenKind::Operator(Operator::Mod),
        TokenKind::Separator(Separator::LBrace),
        TokenKind::Separator(Separator::RBrace),
        TokenKind::Separator(Separator::LParen),
        TokenKind::Separator(Separator::LParen),
        TokenKind::Separator(Separator::RParen),
        TokenKind::Separator(Separator::RParen),
        TokenKind::Operator(Operator::Add),
        TokenKind::Operator(Operator::Sub),
        TokenKind::Operator(Operator::Div),
        TokenKind::Operator(Operator::Mul),
        TokenKind::Operator(Operator::Eq),
        TokenKind::Operator(Operator::Ne),
        TokenKind::Operator(Operator::Lt),
        TokenKind::Separator(Separator::Semicolon),
        TokenKind::Separator(Separator::Semicolon),
        TokenKind::Operator(Operator::Gt),
        TokenKind::Operator(Operator::Le),
        TokenKind::Operator(Operator::Ge),
        TokenKind::Operator(Operator::And),
        TokenKind::Operator(Operator::Or),
        TokenKind::Operator(Operator::Not),
        TokenKind::Operator(Operator::Assign),
        TokenKind::Separator(Separator::Colon),
        TokenKind::Operator(Operator::Sub),
        TokenKind::Separator(Separator::Arrow),
    ];
    compare_output(program, expected, false);
}

#[test]
fn test_error() {
    let program = "\"i am not going to terminate this string literal".to_string();
    let tokens = tokenize(program);
    assert!(tokens.is_err());
    print_err(&tokens.unwrap_err().display(), false);
    let program = r#" 
        // this number is really big lol
        3965264536463463462346243643664326646243623462436643
    "#
    .to_string();
    let tokens = tokenize(program);
    assert!(tokens.is_err());
    print_err(&tokens.unwrap_err().display(), false);
}

#[test]
fn test_keywords() {
    let program = "fn i32 bool string true false if else".to_string();
    let expected = vec![
        TokenKind::Keyword(Keyword::Fn),
        TokenKind::Keyword(Keyword::I32),
        TokenKind::Keyword(Keyword::Bool),
        TokenKind::Keyword(Keyword::String),
        TokenKind::Keyword(Keyword::True),
        TokenKind::Keyword(Keyword::False),
        TokenKind::Keyword(Keyword::If),
        TokenKind::Keyword(Keyword::Else),
    ];
    compare_output(program, expected, false);
}

fn print_err(error_string: &String, should_print: bool) {
    if should_print {
        println!("{}", error_string)
    }
}

fn compare_output(program: String, expected: Vec<TokenKind>, should_print: bool) {
    let tokens = tokenize(program).unwrap();
    assert_eq!(tokens.len(), expected.len(), "Token count mismatch");
    for i in 0..tokens.len() {
        let token = tokens.get(i).unwrap();
        let expect = expected.get(i).unwrap();
        assert_eq!(&token.kind, expect);
        if should_print {
            println!("{}", token.display());
        }
    }
}
