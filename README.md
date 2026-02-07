# "lang"

A functional programming language implemented in Rust.

[![tests](https://github.com/thomascpowell/lang/actions/workflows/rust.yml/badge.svg)](https://github.com/thomascpowell/lang/actions/workflows/rust.yml)

```
// Insertion sort implementation

function insert = fn(x: i32, sorted: list) -> list {
  if (length(sorted) == 0) [x] 
  else if (x <= head(sorted)) x :: sorted 
  else head(sorted) :: insert(x, tail(sorted))
}

function sort = fn(l: list) -> list {
  if (length(l) == 0) []
  else insert(head(l), sort(tail(l)))
}

list test = [2, 7, 4, 3]
println(sort(test))
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
