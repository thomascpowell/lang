use std::{fs, path::Path};

use lang::{interpreter::interpret, lexer::tokenize, parser::parse};

#[test]
fn interpret_simple() {
    interpret(parse(tokenize("i32 digit = 2;".into()).unwrap()).unwrap()).unwrap();
}


// wouldn't pass yet: stdlib not implemented

// #[test]
// fn interpret_demos() {
//     let demo_dir = Path::new("demos");
//     for entry in fs::read_dir(demo_dir).unwrap() {
//         let entry = entry.unwrap();
//         let path = entry.path();
//         let program = fs::read_to_string(&path).unwrap();
//         test_exec(program);
//     }
// }

fn test_exec(src: String) {
    let tokens = tokenize(src).expect("failed to tokenize");
    let ast = parse(tokens).expect("failed to parse");
    interpret(ast).expect("failed to interpret");
}
