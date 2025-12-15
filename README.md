# "lang"

Programming language.

[![tests](https://github.com/thomascpowell/lang/actions/workflows/rust.yml/badge.svg)](https://github.com/thomascpowell/lang/actions/workflows/rust.yml)

```rust
// Code example

def gcd = fn(a: i32, b: i32) -> i32 {
  return if (b == 0) { a } else { gcd(b, a % b) };
};

i32 a = 1071;
i32 b = 462;
println("gcd of ", a, " and ", b, " is: ", gcd(a, b));
```

## Overview
- This is a full programming language written from scratch in Rust
- Currently supports 13 operators, 5 data types, anonymous functions, recursion, and more
- More code examples can be found in `./programs`

## More Info
- Requires only the Rust toolchain (`rust`, `cargo`)
- Documentation (grammar, usage) can be found in `./docs`
- See `Makefile` for running, building, and testing
