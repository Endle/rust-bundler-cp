// Please Read [Rule about third-party code is changing](https://codeforces.com/blog/entry/8790)
extern crate my_lib;
use my_lib::{read, read_ivec, read_uvec};
use my_lib::pr;
// use my_lib::nd;
// use my_lib::multi_queue;
// use my_lib::algo;



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


//maturing
fn smaller_pair(a: u32, b: u32) -> (u32, u32) {
    let smaller = a.min(b);
    let larger = a.max(b);
    return (smaller, larger);
}

// Maturing
fn read_01_vec() -> Vec<u8> {
    let s:String = read!();
    let mut ret = Vec::with_capacity(s.len());

    for c in s.chars() {
        match c {
            '0' => ret.push(0),
            '1' => ret.push(1),
            _ => panic!("Unexpected char")
        }
    }

    ret
}
