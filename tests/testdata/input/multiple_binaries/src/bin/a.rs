// Please Read [Rule about third-party code is changing](https://codeforces.com/blog/entry/8790)
extern crate my_lib;
use my_lib::{read, read_ivec, read_uvec};
use my_lib::pr;



// Currently bundler https://github.com/Endle/rust-bundler/tree/codeforce doesn't support use *


fn solve() -> Option<i32> {
    None
}

fn main() {
    let testcases: i32 = read!();
    // let testcases = 1;
    for _ in 0..testcases { solve_and_print(); }
}

#[inline]
fn solve_and_print() {
    let answer = solve();
    match answer {
        None => (),
        _ => pr::ln(answer.unwrap())
    }
}


