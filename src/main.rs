use lang::{interpreter::interpret, lexer::tokenize, parser::parse};
use std::{env, fs::File, io::Read};
const HELP: &str = include_str!("../docs/help");

/**
* Lang CLI
* */

fn main() {
    let args: Vec<String> = env::args().collect();
    let argument = match args.get(1) {
        None => return print!("{}", HELP),
        Some(x) => x,
    };

    match argument.to_lowercase().as_str() {
        "help" => print!("{}", HELP),
        "source" => handle_source(get_source_from_path(args.get(2))),
        "lexer" => handle_lexer(get_source_from_path(args.get(2))),
        "parser" => handle_parser(get_source_from_path(args.get(2))),
        "run" => handle_run(get_source_from_path(args.get(2))),
        "repl" => handle_repl(),
        x => print!("\nerror: no such argument \"{}\"\n\n{}", x, HELP),
    }
}

/**
* Handlers
* */

fn handle_run(source: String) {
    let tokens = match tokenize(source) {
        Err(e) => return println!("{}", e.display()),
        Ok(t) => t,
    };
    let ast = match parse(tokens) {
        Err(e) => return println!("{}", e.display()),
        Ok(a) => a,
    };
    match interpret(ast) {
        Err(e) => return println!("{}", e.display()),
        Ok(()) => (),
    }
}

fn handle_parser(source: String) {
    let tokens = match tokenize(source) {
        Err(e) => return println!("{}", e.display()),
        Ok(t) => t,
    };
    match parse(tokens) {
        Err(e) => println!("{}", e.display()),
        Ok(a) => a.print_ast(0),
    }
}

fn handle_source(source: String) {
    println!("source:\n{}", source)
}

fn handle_lexer(source: String) {
    match tokenize(source) {
        Err(e) => println!("{}", e.display()),
        Ok(t) => t.iter().for_each(|token| println!("{}", token.display())),
    }
}

fn handle_repl() {
    todo!()
}

/**
* Utility functions
* */

fn get_source_from_path(path: Option<&String>) -> String {
    get_file(path.expect("error: missing required argument <path>"))
}

fn get_file(path: &str) -> String {
    let mut contents = Vec::new();
    File::open(path)
        .expect("error: invalid file path")
        .read_to_end(&mut contents)
        .expect("error: failed to read file");
    String::from_utf8(contents).expect("error: invalid file contents")
}
