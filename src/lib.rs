//! See [README.md](https://github.com/slava-sh/rust-bundler/blob/master/README.md)
use std::io::Read;
use std::mem;
use std::path::Path;

// use syn::ToTokens;
use syn::__private::ToTokens;
use syn::punctuated::Punctuated;
use syn::visit_mut::VisitMut;

use log::{debug, info};
use std::collections::HashMap;


fn get_metadata<P: AsRef<Path>>(package_path: P) -> cargo_metadata::Metadata{
    let manifest_path = package_path.as_ref().join("Cargo.toml");
    let mut cmd = cargo_metadata::MetadataCommand::new();
    cmd.manifest_path(&manifest_path);
    let metadata = cmd.exec().unwrap();
    metadata
}

pub fn bundle_specific_binary<P: AsRef<Path>>(package_path: P, binary_selected:Option<String>,
        bundler_config: HashMap<BundlerConfig, String>) -> String {
    let (bin, lib) = select_bin_and_lib(package_path, binary_selected);
    let base_path = Path::new(&lib.src_path).parent()
        .expect("lib.src_path has no parent");
    let crate_name = &lib.name;

    info!("Expanding binary {:?}", bin.src_path);
    let syntax_tree = read_file(&Path::new(&bin.src_path)).expect("failed to read binary target source");
    let mut file = syn::parse_file(&syntax_tree).expect("failed to parse binary target source");
    let mut expander = Expander::new(base_path, crate_name);
    expander.visit_file_mut(&mut file);
    let code = file.into_token_stream().to_string();
    prettify(code)
}

fn select_bin_and_lib<P: AsRef<Path>>(package_path: P, binary_selected:Option<String>) -> (cargo_metadata::Target, cargo_metadata::Target) {
    let metadata = get_metadata(package_path);
    let targets: &[cargo_metadata::Target] = &metadata.root_package().unwrap().targets;
    let bin = select_binary(targets, binary_selected).clone();
    let lib = get_lib(targets, &bin).clone();

    (bin, lib)
}

fn get_lib<'a>(targets: &'a [cargo_metadata::Target], bin: &'a cargo_metadata::Target) -> &'a cargo_metadata::Target {
    let libs: Vec<_> = targets.iter().filter(|t| target_is(t, "lib")).collect();
    assert!(libs.len() <= 1, "multiple library targets not supported");
    libs.get(0).unwrap_or(&bin)
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

/// Creates a single-source-file version of a Cargo package.
#[deprecated]
pub fn bundle<P: AsRef<Path>>(package_path: P) -> String {
    bundle_specific_binary(package_path, None, HashMap::new())
}

fn target_is(target: &cargo_metadata::Target, target_kind: &str) -> bool {
    target.kind.iter().any(|kind| kind == target_kind)
}

struct Expander<'a> {
    base_path: &'a Path,
    crate_name: &'a str,
}

impl<'a> Expander<'a> {
    fn new(base_path: &'a Path,
           crate_name: &'a str) -> Self {
        Expander {
            base_path,
            crate_name,
        }
    }
    fn expand_items(&self, items: &mut Vec<syn::Item>) {
        debug!("expand_items, count={}", items.len());
        self.expand_extern_crate(items);
        self.expand_use_path(items);
    }

    fn expand_extern_crate(&self, items: &mut Vec<syn::Item>) {
        let mut new_items = vec![];
        for item in items.drain(..) {
            if is_selected_extern_crate(&item, self.crate_name) {
                info!(
                    "expanding crate(lib.rs) {} in {}",
                    self.crate_name,
                    self.base_path.to_str().unwrap()
                );
                let code =
                    read_file(&self.base_path.join("lib.rs")).expect("failed to read lib.rs");
                let lib = syn::parse_file(&code).expect("failed to parse lib.rs");
                debug!("parsed lib: {}", debug_str_items(&lib.items));
                new_items.extend(lib.items);
            } else {
                new_items.push(item);
            }
        }
        *items = new_items;
    }

    fn expand_use_path(&self, items: &mut Vec<syn::Item>) {
        let mut new_items = vec![];
        for item in items.drain(..) {
            if !is_use_path(&item, self.crate_name) {
                new_items.push(item);
            }
        }
        *items = new_items;
    }

    fn expand_mods(&self, item: &mut syn::ItemMod) {
        if item.content.is_some() {
            return;
        }
        let name = item.ident.to_string();
        let other_base_path = self.base_path.join(&name);
        let (base_path, code) = vec![
            (self.base_path, format!("{}.rs", name)),
            (&other_base_path, String::from("mod.rs")),
        ].into_iter()
            .flat_map(|(base_path, file_name)| {
                read_file(&base_path.join(file_name)).map(|code| (base_path, code))
            })
            .next()
            .expect("mod not found");
        info!("expanding mod {} in {}", name, base_path.to_str().unwrap());
        let mut file = syn::parse_file(&code).expect("failed to parse file");
        Expander {
            base_path,
            crate_name: self.crate_name,
        }.visit_file_mut(&mut file);
        item.content = Some((Default::default(), file.items));
    }

    fn expand_crate_path(&mut self, path: &mut syn::Path) {
        if path_starts_with(path, self.crate_name) {
            let new_segments = mem::replace(&mut path.segments, Punctuated::new())
                .into_pairs()
                .skip(1)
                .collect();
            path.segments = new_segments;
        }
    }


}


impl<'a> VisitMut for Expander<'a> {
    fn visit_file_mut(&mut self, file: &mut syn::File) {
        debug!("Custom visit_file_mut, item: {}", debug_str_items(&file.items));
        for it in &mut file.attrs {
            self.visit_attribute_mut(it)
        }
        debug!("{:?}", file);
        self.expand_items(&mut file.items);
        for it in &mut file.items {
            self.visit_item_mut(it)
        }
        // eprintln!("File attr {:?}=========", & file.attrs);
        // eprintln!("File items {:?}", & file.items);
    }

    fn visit_item_mod_mut(&mut self, item: &mut syn::ItemMod) {
        for it in &mut item.attrs {
            self.visit_attribute_mut(it)
        }
        self.visit_visibility_mut(&mut item.vis);
        self.visit_ident_mut(&mut item.ident);
        self.expand_mods(item);
        if let Some(ref mut it) = item.content {
            for it in &mut (it).1 {
                self.visit_item_mut(it);
            }
        }
    }

    fn visit_path_mut(&mut self, path: &mut syn::Path) {
        self.expand_crate_path(path);
        for mut el in Punctuated::pairs_mut(&mut path.segments) {
            let it = el.value_mut();
            self.visit_path_segment_mut(it)
        }
    }
}

fn is_selected_extern_crate(item: &syn::Item, crate_name: &str) -> bool {
    if let syn::Item::ExternCrate(ref item) = *item {
        if item.ident == crate_name {
            return true;
        }
    }
    false
}

fn path_starts_with(path: &syn::Path, segment: &str) -> bool {
    if let Some(el) = path.segments.first() {
        if el.ident == segment {
            return true;
        }
    }
    false
}

fn is_use_path(item: &syn::Item, first_segment: &str) -> bool {
    if let syn::Item::Use(ref item) = *item {
        if let syn::UseTree::Path(ref path) = item.tree {
            if path.ident == first_segment {
                return true;
            }
        }
    }
    false
}

fn read_file(path: &Path) -> Option<String> {
    let mut buf = String::new();
    std::fs::File::open(path).ok()?.read_to_string(&mut buf).ok()?;
    Some(buf)
}

#[cfg(feature = "inner_rustfmt")]
fn prettify(code: String) -> String {
    use rustfmt_nightly::{Input, Session, Config, EmitMode, Verbosity};
    let mut out = Vec::with_capacity(code.len() * 2);
    {
        let mut config = Config::default();
        config.set().emit_mode(EmitMode::Stdout);
        config.set().verbose(Verbosity::Quiet);
        let input = Input::Text(code.into());
        let mut session = Session::new(config, Some(&mut out));
        session.format(input).expect("rustfmt failed");
    }
    String::from_utf8(out).unwrap()
}

#[cfg(not(feature = "inner_rustfmt"))]
fn prettify(code: String) -> String {
    use std::io::Write;
    use std::process;
    let mut command = process::Command::new("rustfmt")
        .stdin(process::Stdio::piped())
        .stdout(process::Stdio::piped())
        .stderr(process::Stdio::piped())
        .spawn()
        .expect("Failed to spawn rustfmt process");
    {
        let mut stdin = command.stdin.take().unwrap();
        write!(stdin, "{}", code).unwrap();
    }
    let out = command.wait_with_output().unwrap();
    if !out.status.success() {
        let error_code = match out.status.code() {
            Some(x) => x.to_string(),
            None => String::from("Error_Code_None")
        };
        let stderr = out.stderr;
        let stderr = String::from_utf8(stderr)
            .unwrap_or( String::from("Invalid stderr String") );
       panic!("rustfmt failed, code={}\nstderr: {}", error_code, stderr);
    }
    let stdout = out.stdout;
    String::from_utf8(stdout).unwrap()
}


// Debug toolkits

fn debug_str_items(items: &Vec<syn::Item>) -> String {
    // let x = 5i32;
    // let y = x.to_string();
    //HIGHLY TODO
    let mut builder = simple_string_builder::Builder::new();
    builder.append("len=");
    // builder.append(items.len());
    builder.append(items.len().to_string());
    builder.append(" ");
    // result += &*items.len().to_string();
    for it in items {
        builder.append(" / ");
        builder.append(debug_str_item(it));
    }
    builder.try_to_string().unwrap()
    // let mut result = String::new();

    // result += "len=";
}

fn debug_str_item(it: &syn::Item) -> String {
    let refstr:&str = match it {
        syn::Item::ExternCrate(_e) => {
            // eprintln!("{:?}", e); //TODO-> too hacky
            "extern_crate"
        },
        syn::Item::Use(_e) => {
            // eprintln!("{:?}", e); //TODO-> too hacky
            "use"
        },
        syn::Item::Fn(_e) => {
            // eprintln!("{:?}", e); //TODO-> too hacky
            "Fn"
        },
        syn::Item::Mod(e) => {
            e.ident.to_string();
            eprintln!("{:?}", e); //TODO-> too hacky
            // return "Mod(";
            return format!("Mod ({})", e.ident.to_string());
        },
        _ => {
            // eprintln!("{:?}", it); //TODO-> too hacky
            "Others"
        }
    };
    String::from(refstr)
}

pub enum BundlerConfig {
    RemoveUnusedModInLib,
}
