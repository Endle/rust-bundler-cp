extern crate rust_bundler_cp;

use clap::App;
use std::collections::HashMap;
use std::process;

use rust_bundler_cp::BundlerConfig;

use log::warn;

fn main() {
    env_logger::builder()
        .format_timestamp(None)
        .format_target(false)
        .init();

    let matches = App::new("rust_bundler_cp")
        .version("0.3")
        .author("Zhenbo Li")
        .about("Creates a single-source-file version of a Cargo package.")
        .args_from_usage("-i, --input=[PATH] 'REQUIRED. Path to a cargo directory' ")
        .args_from_usage("-o, --output=[FILE] 'If not specified, result would be written to STDIN'")
        .args_from_usage("-b --binary=[NAME] 'If multiple [[bin]] defined in Cargo.toml, this field is required'")
        .args_from_usage("--remove_unused_mod 'If a pub mod statement in lib.rs is not used in selected bin, it would be removed EXPERIMENTAL!!'")
        .get_matches();

    let path: String = match matches.value_of("input") {
        None => {
            eprintln!("Error! Input path have to be specified");
            process::exit(1);
        }
        Some(v) => String::from(v),
    };

    let binary_selected = match matches.value_of("binary") {
        None => None,
        Some(v) => Some(String::from(v))
    };


    let mut config: HashMap<BundlerConfig, String> = HashMap::new();

    if matches.is_present("remove_unused_mod") {
        warn!("Experimental function remove_unused_mod enabled");
        config.insert(BundlerConfig::RemoveUnusedModInLib, String::new());
    }

    let code = rust_bundler_cp::bundle_specific_binary(path, binary_selected, config);

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
