use std::io::Read;
use std::path::Path;
#[test]
fn parse_test_io() {
    let code_path_str = "tests/testdata/test_io.rs";
    let code_path = Path::new(code_path_str);
    // let base_path = Path::new(&lib.src_path).parent()
    //     .expect("lib.src_path has no parent");

    let lib_rs_code = read_file(code_path).expect("failed to read lib.rs");
    // debug!("Loaded lib.rs: {}", lib_rs_code.len());
    let lib = syn::parse_file(&lib_rs_code).expect("failed to parse lib.rs");
    print!("parsed lib: {}", lib.items.len());
}

fn read_file(path: &Path) -> Option<String> {
    let mut buf = String::new();
    std::fs::File::open(path)
        .ok()?
        .read_to_string(&mut buf)
        .ok()?;
    Some(buf)
}

#[test]
fn parse_pub_mod() {
    let lib_rs_code = "pub mod algo;";
    let lib = syn::parse_file(lib_rs_code).expect("failed to parse lib.rs");
    print!("parsed lib: {}", lib.items.len());
}
