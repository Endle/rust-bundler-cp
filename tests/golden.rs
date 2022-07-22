extern crate goldenfile;
extern crate rust_bundler_cp;

use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use goldenfile::Mint;

const INPUT_DIR: &str = "tests/testdata/input";
const OUTPUT_DIR: &str = "tests/testdata/output";
const MULTIPLE_BINARIES_TEST_NAME: &str = "multiple_binaries";

#[test]
fn loop_test_cases() {
    let mut mint = Mint::new(OUTPUT_DIR);
    for entry in fs::read_dir(INPUT_DIR).expect("read_dir failed") {
        let path = entry.expect("Invalid path").path();
        let path_str = path.as_path().display().to_string();
        if path_str.contains(MULTIPLE_BINARIES_TEST_NAME) {
            eprintln!("Skip multiple_binaries");
            continue;
        }

        golden(&mut mint, path);
    }
}

fn golden(mint: &mut Mint, input_path: PathBuf) {
    let input_name = input_path.file_name().expect("no file name");
    let output_name = Path::new(input_name).with_extension("rs");
    let mut output_file = mint
        .new_goldenfile(output_name)
        .expect("new_goldenfile failed");
    let output = rust_bundler_cp::bundle_specific_binary(&input_path, None, HashMap::new());
    write!(output_file, "{}", output).expect("write! failed");
    output_file.flush().expect("flush failed");
}
