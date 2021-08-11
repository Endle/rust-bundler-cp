# rust-bundler-cp

This project is based on [rust-bundler](https://github.com/slava-sh/rust-bundler/)

Creates a single-source-file version of a Cargo package.

## Features

* Replaces `extern crate my_lib;` in `main.rs` with the contents of `lib.rs`.
* Expands `mod my_mod;` declarations into `mod my_mod { ... }` blocks.

## Example

Input:
```rust
// src/internal.rs:
pub fn hello_world() {
    println!("Hello, world!");
}

// src/lib.rs:
mod internal;
pub use internal::hello_world;

// src/main.rs:
extern crate example;
fn main() {
    example::hello_world();
}
```

Output:
```rust
mod internal {
    pub fn hello_world() {
        println!("Hello, world!");
    }
}
pub use internal::hello_world;
fn main() {
    hello_world();
}
```

More examples in [tests/testdata](https://github.com/slava-sh/rust-bundler/tree/master/tests/testdata).

## Usage

Install:
```sh
$ cargo install rust_bundler_cp
```

Run:
```sh
$ rust_bundler_cp --input path/to/project >output.rs
```



## Similar Projects
* This project is based on [slava-sh /rust-bundler](https://github.com/slava-sh/rust-bundler)
* [lpenz/rust-sourcebundler](https://github.com/lpenz/rust-sourcebundler)
  is based on regular expressions, whereas this project manipulates the syntax tree
* [MarcosCosmos/cg-rust-bundler](https://github.com/MarcosCosmos/cg-rust-bundler)
* [golang.org/x/tools/cmd/bundle](https://godoc.org/golang.org/x/tools/cmd/bundle) for Go
