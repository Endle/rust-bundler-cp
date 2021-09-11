extern crate rust_bundler_cp;

use std::process;
use clap::App;

fn main() {
    let matches = App::new("rust_bundler_cp")
        .version("0.2")
        .author("Zhenbo Li")
        .about("Creates a single-source-file version of a Cargo package.")
        .args_from_usage("-i, --input=[PATH] 'REQUIRED. Path to a cargo directory' ")
        .args_from_usage("-o, --output=[FILE] 'If not specified, result would be written to STDIN'")
        .args_from_usage("-b --binary=[NAME] 'If multiple [[bin]] defined in Cargo.toml, this field is required'")
        .get_matches();


    let path: String = match matches.value_of("input") {
        None => {
            eprintln!("Error! Input path have to be specified");
            process::exit(1);
        }
        Some(v) => String::from(v)
    };

    let binary_selected = match matches.value_of("binary") {
        None => None,
        Some(v) => Some(String::from(v))
    };

    let code = rust_bundler_cp::bundle_specific_binary(path, binary_selected);

    match matches.value_of("output") {
        None => {
            println!("{}", code);
        }
        Some(v) => {
            let v: String = String::from(v);
            std::fs::write(&v, code).expect("Unable to write file");
            eprintln!("Bundled rust code written to: {}", &v);
        }
    };
}
