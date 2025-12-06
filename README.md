# "Lang"

Toy programming language written in Rust.

[![tests](https://github.com/thomascpowell/lang/actions/workflows/rust.yml/badge.svg)](https://github.com/thomascpowell/lang/actions/workflows/rust.yml)

```rust
// Basic code example

def min = fn(a: i32, b: i32) -> string {
  return if (a < b) { "a is smaller" } else { "b is smaller" };
};

println(min(1, 100));
```

## Features
- Helpful error messages with exact token locations
- Anonymous functions, if expressions, and recursion
- Strict and explicit type system

## Details
- Lexer: Simple, supports comments
- Parser: Generates AST, enforces grammar
- Interpreter: Executes the AST

## Next Steps
- Implement REPL and CLI
- Expand standard library
- General performance optimizations
- Rework internal error types and methods
