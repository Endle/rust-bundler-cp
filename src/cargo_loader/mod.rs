use std::path::Path;
use log::error;

pub fn select_bin_and_lib<P: AsRef<Path>>(package_path: P, binary_selected:Option<String>) -> (cargo_metadata::Target, cargo_metadata::Target) {
    let metadata = get_metadata(package_path);
    let root_package = metadata.root_package().unwrap();
    let targets: &[cargo_metadata::Target] = &root_package.targets;
    let bin = select_binary(targets, binary_selected).clone();
    let lib = get_lib(targets, &bin).clone();

    if root_package.dependencies.len() > 0 {
        error!("Unsupported: {} has more than one dependency", &root_package.name);
    }
    (bin, lib)
}

fn get_metadata<P: AsRef<Path>>(package_path: P) -> cargo_metadata::Metadata{
    let manifest_path = package_path.as_ref().join("Cargo.toml");
    let mut cmd = cargo_metadata::MetadataCommand::new();
    cmd.manifest_path(&manifest_path);
    cmd.exec().unwrap()
}

fn select_binary(targets: &[cargo_metadata::Target], select: Option<String>) -> &cargo_metadata::Target {
    let bins: Vec<_> = targets.iter().filter(|t| target_is(t, "bin")).collect();
    assert_ne!(bins.len(), 0, "no binary target found");

    if select.is_none() {
        // println!("{:?}", &bins);
        if bins.len() != 1 {
            panic!("If there are multiple binary targets, MUST SPECIFY which one to use");
        }

        return bins[0];
    }
    let binary_name = select.unwrap();
    for bin in bins {
        if bin.name.eq(&binary_name) {
            return bin;
        }
    }
    panic!("Can't find binary {}", binary_name);
}

fn target_is(target: &cargo_metadata::Target, target_kind: &str) -> bool {
    target.kind.iter().any(|kind| kind == target_kind)
}

fn get_lib<'a>(targets: &'a [cargo_metadata::Target], bin: &'a cargo_metadata::Target) -> &'a cargo_metadata::Target {
    let libs: Vec<_> = targets.iter().filter(|t| target_is(t, "lib")).collect();
    assert!(libs.len() <= 1, "multiple library targets not supported");
    libs.get(0).unwrap_or(&bin)
}

