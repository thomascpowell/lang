use std::{env, fs::File, io::Read};

use lang::lexer::tokenize;

/**
* Lang CLI
* */

// CLI is a work-in-progress, see Makefile for running tests

const HELP: &str = include_str!("../docs/help");

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
        x => print!("\nerror: no such argument \"{}\"\n\n{}", x, HELP),
    }
}

/**
* Handlers
* */

fn handle_source(source: String) {
    println!("source:\n{}", source)
}

fn handle_lexer(source: String) {
    match tokenize(source) {
        Err(e) => println!("{}", e.display()),
        Ok(t) => t.iter().for_each(|token| println!("{}", token.display())),
    }
}

/**
* utility functions
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
