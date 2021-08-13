extern crate assert_cli;

use assert_cli::Assert;
use std::fs::{DirEntry, read_dir};

#[test]
fn usage() {
    cli_bundle(&[],
               false,
               &["Error! Input path have to be specified"])
}

#[test]
fn bundle_self() {
    cli_bundle(&["--input", "."],
               true,
               &["pub fn bundle<", "let code = bundle("])

}

#[test]
fn multiple_binaries() {
    cli_bundle(&["--input", "testdata/input/multiple_binaries"],
               true,
               &["pub fn bundle<", "let code = bundle("])

}

fn cli_bundle(args: &[&str], success: bool, expect_output: &[&str]) -> () {
    let mut assert = Assert::main_binary().with_args(args);
    if success {
        assert = assert.succeeds();
    }  else {
        assert = assert.fails();
    }
    for s in expect_output {
        let ss = *s;
        if success {
            assert = assert.stdout().contains(ss);
        } else {
            assert = assert.stderr().contains(ss);
        }
    }
}