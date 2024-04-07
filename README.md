#  [![Stand With Ukraine](https://raw.githubusercontent.com/vshymanskyy/StandWithUkraine/main/badges/StandWithUkraine.svg)](https://stand-with-ukraine.pp.ua) rust-bundler-cp



This project is based on [rust-bundler](https://github.com/slava-sh/rust-bundler/)

Creates a single-source-file version of a Cargo package. It's designed for Competitive Programming like Codeforces.



## Features

* Uses [Syn](https://docs.rs/syn/latest/syn/) to parse and manipulate a complete syntax tree, instead of doing replacement with regular expression.  
* Replaces `extern crate my_lib;` in `main.rs` with the contents of `lib.rs`.
* Expands `mod my_mod;` declarations into `mod my_mod { ... }` blocks.
* [rustfmt](https://github.com/rust-lang/rustfmt) needs to be available in `PATH`, i.e. `dnf install rustfmt`. I'll consider bundle `rustfmt` in my code later.
* Unsupported: External `[dependencies]` in `Cargo.toml`  

## Example
 
[Endle's codeforces template](https://github.com/Endle/rust_codeforce_template) is co-evoloved with [rust-bundler-cp](https://github.com/Endle/rust-bundler-cp), and is considered as the example of it.  

## Usage

Install:
```sh
$ cargo install rust_bundler_cp
```

Run:
```sh
$ rust_bundler_cp --input path/to/project >output.rs
$ rust_bundler_cp --input path/to/project --binary a
```



## Similar Projects
* This project is based on [slava-sh /rust-bundler](https://github.com/slava-sh/rust-bundler)
* [lpenz/rust-sourcebundler](https://github.com/lpenz/rust-sourcebundler)
  is based on regular expressions, whereas this project manipulates the syntax tree
* [MarcosCosmos/cg-rust-bundler](https://github.com/MarcosCosmos/cg-rust-bundler)
* [golang.org/x/tools/cmd/bundle](https://godoc.org/golang.org/x/tools/cmd/bundle) for Go
