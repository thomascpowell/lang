# "Lang"

An unfinished (and untitled) programming language written in Rust.

[![tests](https://github.com/thomascpowell/lang/actions/workflows/rust.yml/badge.svg)](https://github.com/thomascpowell/lang/actions/workflows/rust.yml)

```rust

def min = fn(a: i32, b: i32) -> string {
  return if (a > b) { "a is bigger" } else { "b is bigger" };
};

print(min(1, 2));
```

## Features
- Helpful error messages with exact token locations
- Anonymous functions, if expressions, and recursion
- Strict and explicit type system

## Details
- Lexer: Simple, supports comments
- Parser: Generates AST, enforces grammar
- Interpreter: Executes the AST

## Todo
- Expand standard library
- Bug fixes relating to booleans
- Rework internal error types and methods
