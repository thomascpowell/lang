use lang::lexer::tokenize;
use lang::lexer::token::*;

#[test]
fn test_full_parse() {
    let program = "
        fn main() { 
            //some comment
            string x = \"hello\";
            i32 y = 4 / 2;
        }
        "
    .to_string();

    let expected = vec![
        TokenKind::Keyword(Keyword::Fn),
        TokenKind::Identifier("main".to_string()),
        TokenKind::Separator(Separator::LParen),
        TokenKind::Separator(Separator::RParen),
        TokenKind::Separator(Separator::LBrace),
        TokenKind::Comment("some comment".to_string()),
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
        TokenKind::Literal(Literal::Int(2)),
        TokenKind::Separator(Separator::Semicolon),
        TokenKind::Separator(Separator::RBrace),
    ];
    compare_output(program, expected, false);

}

#[test]
fn test_operators() {
    let program = "+ -/ * ==!= < > <=>= && || ! =".to_string();
    let expected = vec![
        TokenKind::Operator(Operator::Add),
        TokenKind::Operator(Operator::Sub),
        TokenKind::Operator(Operator::Div),
        TokenKind::Operator(Operator::Mul),
        TokenKind::Operator(Operator::Eq),
        TokenKind::Operator(Operator::Ne),
        TokenKind::Operator(Operator::Lt),
        TokenKind::Operator(Operator::Gt),
        TokenKind::Operator(Operator::Le),
        TokenKind::Operator(Operator::Ge),
        TokenKind::Operator(Operator::And),
        TokenKind::Operator(Operator::Or),
        TokenKind::Operator(Operator::Not),
        TokenKind::Operator(Operator::Assign),
    ];
    compare_output(program, expected, false);
}

#[test]
fn test_error() {
    let program = "\"i am not going to terminal this string literal".to_string();
    let tokens = tokenize(program);
    assert!(tokens.is_err());
    println!("{}", tokens.unwrap_err().display());
    let program = "
        // this number is really big lol
        3965264536463463462346243643664326646243623462436643
    ".to_string();
    let tokens = tokenize(program);
    assert!(tokens.is_err());
    println!("{}", tokens.unwrap_err().display())


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
