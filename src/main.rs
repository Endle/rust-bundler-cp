extern crate rust_bundler_cp;

use std::process;
use clap::App;

fn main() {
    let matches = App::new("rust_bundler_cp")
        .version("0.2")
        .author("Zhenbo Li")
        .about("Creates a single-source-file version of a Cargo package.")
        .arg("-i, --input=[PATH] 'REQUIRED. Path to a cargo directory' ")
        .arg("-o, --output=[FILE] 'If not specified, result would be written to STDIN'")
        .get_matches();


    let path:String = match matches.value_of_t("input") {
             // => println!("{}", x),
            Err(_)       => {
                eprintln!("Error! Input path have to be specified");
                process::exit(1);
            }
            Ok(v) => v
    };
    // eprintln!("Input path = {}", path);

    let code = rust_bundler_cp::bundle(path);

    match matches.value_of_t("output") {
        Err(_)  => {
            println!("{}", code);
        }
        Ok(v) => {
            let v:String = v;
            std::fs::write(&v, code).expect("Unable to write file");
            eprintln!("Bundled rust code written to: {}", &v);
        }
    };
}