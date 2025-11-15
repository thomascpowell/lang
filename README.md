# "Lang"

An untitled and unfinished programming language. 

[![tests](https://github.com/thomascpowell/lang/actions/workflows/rust.yml/badge.svg)](https://github.com/thomascpowell/lang/actions/workflows/rust.yml)


```rust
// readme_example.lang

string min = fn(a: i32, b: i32) {
  return if (a > b) { "a is bigger" } else { "b is bigger" };
};

print(min(1, 2));
```

## Features
- Helpful error messages with exact token locations
- Anonymous functions, if expressions, recursion
- Strict and explicit type system

## Details
- Lexer: Simple, supports comments
- Parser: Generates AST, enforces grammar
- Interpreter: WIP, supports variable scopes
