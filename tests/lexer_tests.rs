use lang::lexer::*;
use lang::token_types::*;

#[test]
fn test_display_parse() {
    let program = "
        fn main() { 
            string x = \"hello\";
            i32 y = 4 / 2;
        }
        ".to_string();
    let tokens = tokenize(program);

    match tokens {
        Ok(t) => {
            for token in t {
                println!("{}", token.display());
            }
        }
        Err(e) => {
            println!("{}", e.display())
        }
    }

}
