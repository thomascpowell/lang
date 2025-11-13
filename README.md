# "Lang"

An untitled and unfinished programming language. 

[![tests](https://github.com/thomascpowell/lang/actions/workflows/rust.yml/badge.svg)](https://github.com/thomascpowell/lang/actions/workflows/rust.yml)


```rust
i32 hello = fn(name: string) {
  print("hello ");
  print(name);
  return 0;
};

hello("name");
```

## Features
- Helpful error messages with exact token locations
- Anonymous functions, if expressions, recursion
- Strict and explicit type system

## Details
- Lexer: Simple, supports comments
- Parser: Generates AST, enforces grammar
- Interpreter: WIP, supports variable scopes
