extern crate rust_bundler_cp;

use std::env;
use std::process;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: bundle path/to/project");
        process::exit(1);
    }
    let code = rust_bundler_cp::bundle(&args[1]);
    println!("{}", code);
}
