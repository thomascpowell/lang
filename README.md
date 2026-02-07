# "lang"

A functional programming language implemented in Rust.

[![tests](https://github.com/thomascpowell/lang/actions/workflows/rust.yml/badge.svg)](https://github.com/thomascpowell/lang/actions/workflows/rust.yml)

```
// Example

function apply_twice = fn(f: function, x: i32) -> i32 {
  return f(f(x));
}

function add = fn(n: i32) -> function {
  return fn(x: i32) -> i32 { x + n };
}

function add10 = add(10);

println(apply_twice(add10, 10)); // prints 30
```

## Overview
- Basic documentation (grammar and usage) can be found in `./docs`
- See `Makefile` for running, building, and testing
- More code examples can be found in `./programs`

## Features
- Lexical scopes 
    - Closures capture their environment
    - Recursion and shadowing are fully supported
- First-class functions
    - Functions are values created by anonymous function expressions
    - Functions can be assigned to variables, passed as arguments, and returned
- Immutable lists
    - Implemented as a linked list
    - Constructed using list literals or `::`
- Standard library
    - Standard library functions are automatically in scope
    - Key functions include `assert`, `println`, and `read`
- Helpful debugging tools
    - Errors include a clear explanation and source position
    - The CLI can print the AST or token list
