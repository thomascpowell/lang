use std::env;

/**
* Lang CLI
* */

// CLI is a work-in-progress, see Makefile for running tests

const HELP: &str = include_str!("../docs/help");

fn main() {
    let mut args: Vec<String> = env::args().collect();

    let argument = match args.pop() {
        None => return print!("{}", HELP),
        Some(x) => x,
    };

    match argument.to_lowercase().as_str() {
        "help" => print!("{}", HELP),
        x => print!("error: no such argument \"{}\"\n\n{}", x, HELP)
    }
}
