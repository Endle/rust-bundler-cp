use std::fs::{DirEntry, read_dir};
use std::process::Command;

#[test]
fn loop_cargos() {
    const INPUT_DIR: &str = "tests/testdata/input";
    for entry in read_dir(INPUT_DIR).expect("read_dir failed") {
        validate(entry.expect("Not valid path"));
    }
}

fn validate(path: DirEntry) {
    let pathbuf = path.path();
    eprintln!("Building {:?}", &pathbuf);


    let status = Command::new("cargo")
        .current_dir(pathbuf)
        .arg("build")
        .arg("-v")
        .status()
        .expect("failed to execute process");

    eprintln!("process finished with: {}", status);
    assert!(status.success());
}
