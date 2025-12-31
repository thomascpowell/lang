# Lang

A small, functional programming language written from scratch in Rust.

[![tests](https://github.com/thomascpowell/lang/actions/workflows/rust.yml/badge.svg)](https://github.com/thomascpowell/lang/actions/workflows/rust.yml)

```
// Example

function apply_twice = fn(f: function, x: i32) -> i32 {
  return f(f(x));
}

function add = fn(n: i32) -> function {
  return fn(x: i32) -> i32 { return x + n };
}

function add10 = add(10);

println(apply_twice(add10, 10)); // prints 30
```

## Overview
- Basic documentation (grammar and usage) can be found in `./docs`
- See `Makefile` for running, building, and testing
- More code examples can be found in `./programs`

## Features
- Runtime-validated type system
    - Basic data types (`f32`, `i32`, `string`, etc.)
    - Types are fully validated 
- Lexical scoping with closures 
    - Closures capture their environment
    - Inner scopes can redefine identifiers
    - Recursion works correctly
- First-class function support
    - Functions are values created by anonymous function expressions
    - Functions can be assigned to variables, passed as arguments, and returned
- Standard library with native functions
    - Standard library functions are automatically in scope
    - Key functions include `print`, `println`, and `read`
- Helpful error messages
    - Errors include a clear message and show what was found
    - Errors also display source position (line and column)
