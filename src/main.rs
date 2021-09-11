extern crate rust_bundler_cp;

use std::process;
use clap::App;
use std::collections::HashMap;

use rust_bundler_cp::BundlerConfig;

use log::{warn};

fn main() {
    env_logger::builder()
        .format_timestamp(None)
        .format_target(false)
        .init();

    let matches = App::new("rust_bundler_cp")
        .version("0.3")
        .author("Zhenbo Li")
        .about("Creates a single-source-file version of a Cargo package.")
        .arg("-i, --input=[PATH] 'REQUIRED. Path to a cargo directory' ")
        .arg("-o, --output=[FILE] 'If not specified, result would be written to STDIN'")
        .arg("-b --binary=[NAME] 'If multiple [[bin]] defined in Cargo.toml, this field is required'")
        .arg("--remove_unused_mod 'If a pub mod statement in lib.rs is not used in selected bin, it would be removed EXPERIMENTAL!!'")
        .get_matches();


    let path:String = match matches.value_of_t("input") {
        Err(_)       => {
            eprintln!("Error! Input path have to be specified");
            process::exit(1);
        }
        Ok(v) => v
    };
    // eprintln!("Input path = {}", path);

    let binary_selected = match matches.value_of_t("binary") {
        Err(_)       => {
            None
        }
        Ok(v) => Some(v)
    };


    let mut config: HashMap<BundlerConfig, String> = HashMap::new();

    if matches.is_present("remove_unused_mod") {
        warn!("Experimental function remove_unused_mod enabled");
        config.insert(BundlerConfig::RemoveUnusedModInLib, String::new());
    }

    let code = rust_bundler_cp::bundle_specific_binary(path, binary_selected, config);

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
