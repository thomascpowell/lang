# "Lang"

Toy programming language written in Rust.

[![tests](https://github.com/thomascpowell/lang/actions/workflows/rust.yml/badge.svg)](https://github.com/thomascpowell/lang/actions/workflows/rust.yml)

```rust
// Code example

def min = fn(a: i32, b: i32) -> string {
  return if (a < b) { "a is smaller" } else { "b is smaller" };
};

println(min(1, 100));
```

## Features
- Helpful error messages with exact token locations
- Strict and explicit type system
- Simple CLI for executing source code
