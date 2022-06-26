use std::io::Read;
use std::mem;
use std::path::Path;

// use syn::ToTokens;
use syn::__private::ToTokens;
use syn::punctuated::Punctuated;
use syn::visit_mut::VisitMut;

use log::{debug, info, error};
use std::collections::{HashMap, HashSet};

mod cargo_loader;

pub fn bundle_specific_binary<P: AsRef<Path>>(package_path: P, binary_selected:Option<String>,
        bundler_config: HashMap<BundlerConfig, String>) -> String {
    let (bin, lib) = cargo_loader::select_bin_and_lib(package_path, binary_selected);
    let base_path = Path::new(&lib.src_path).parent()
        .expect("lib.src_path has no parent");
    let crate_name = &lib.name;

    info!("Expanding binary {:?}", bin.src_path);
    let syntax_tree = read_file(&Path::new(&bin.src_path)).expect("failed to read binary target source");
    let mut file = syn::parse_file(&syntax_tree).expect("failed to parse binary target source");
    let mut expander = Expander::new(base_path, crate_name);
    if bundler_config.contains_key(&BundlerConfig::RemoveUnusedModInLib) {
        expander.set_pub_mod_allow_list(&file);
    }
    expander.visit_file_mut(&mut file);
    let code = file.into_token_stream().to_string();
    prettify(code)
}

/// Creates a single-source-file version of a Cargo package.
#[deprecated]
pub fn bundle<P: AsRef<Path>>(package_path: P) -> String {
    bundle_specific_binary(package_path, None, HashMap::new())
}

struct Expander<'a> {
    base_path: &'a Path,
    crate_name: &'a str,
    remove_unused_mod_in_lib: bool,
    allow_list_mod_in_lib: HashSet<String>,
}

impl<'a> Expander<'a> {
    fn new(base_path: &'a Path, crate_name: &'a str) -> Expander<'a> {
        Expander {
            base_path,
            crate_name,
            remove_unused_mod_in_lib: false,
            allow_list_mod_in_lib: HashSet::new(),
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
                info!("expanding crate(lib.rs) {} in {}",
                    self.crate_name, self.base_path.to_str().unwrap());
                let lib_rs_code =
                    read_file(&self.base_path.join("lib.rs")).expect("failed to read lib.rs");
                debug!("Loaded lib.rs: {}", lib_rs_code.len());
                let lib = syn::parse_file(&lib_rs_code);
                let lib = match lib {
                    Ok(x) => x,
                    Err(e) => {
                        error!("syn lib failed {:?}", e);
                        std::process::exit(1);
                    }
                };
                // .expect("failed to parse lib.rs");
                debug!("parsed lib: {}", debug_str_items(&lib.items));
                if self.remove_unused_mod_in_lib {
                    debug!("Remove unused mod in lib.rs");
                    for it in lib.items {
                        if self.is_allowed(&it) {
                            new_items.push(it);
                        } else {
                            debug!("mod {} has been skipped", it.to_token_stream().to_string());
                        }
                    }
                } else {
                    new_items.extend(lib.items);
                }

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
        Expander::new(base_path, self.crate_name)
            .visit_file_mut(&mut file);
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

    fn set_pub_mod_allow_list(&mut self, file: &syn::File) {
        debug!("set_pub_mod_allow_list");

        self.remove_unused_mod_in_lib = true;
        for it in &file.items {
            if let syn::Item::Use(e) = it {
                let mods = extract_mods_name(&e.tree);
                for x in mods {
                    self.allow_list_mod_in_lib.insert(x);
                }
            }
        }
        debug!("set_pub_mod_allow_list result: {:?}", &self.allow_list_mod_in_lib);
    }

    fn is_allowed(&self, it: &syn::Item) -> bool {
        match it {
            syn::Item::Mod(e) => {
                let name = e.ident.to_string();
                debug!("Checking if {} ({}) is_allowed", e.to_token_stream().to_string(), &name);
                self.allow_list_mod_in_lib.contains(&name)
                // true
            },
            _ => {
                true
            }
        }
    }
}

fn extract_mods_name(item: &syn::UseTree) -> Vec<String> {
    let mut result = Vec::new();

    match item {
        syn::UseTree::Path(p) => {
            //TODO should check  ident: Ident(my_lib) here
            return extract_mods_name(&*p.tree)
        },
        syn::UseTree::Group(g) => {
            for c in &g.items {
                let mut mods = extract_mods_name(c);
                result.append(&mut mods);
            }
        },
        syn::UseTree::Name(n) => {
            result.push(n.ident.to_string());
        },
        _ => {
            error!("Unexpected Tree element {}", item.to_token_stream().to_string());
        }
    }

    debug!("extract_used_mods: {}, result: {:?}", item.to_token_stream().to_string(), &result);
    result
}


impl<'a> VisitMut for Expander<'a> {
    fn visit_file_mut(&mut self, file: &mut syn::File) {
        debug!("Custom visit_file_mut, item: {}", debug_str_items(&file.items));
        for it in &mut file.attrs {
            self.visit_attribute_mut(it)
        }
        // debug!("{:?}", file);
        self.expand_items(&mut file.items);
        for it in &mut file.items {
            self.visit_item_mut(it)
        }

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

fn debug_str_items(items: &[syn::Item]) -> String {
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
            // eprintln!("{:?}", e); //TODO-> too hacky
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

#[derive(PartialEq, Eq, Hash)]
pub enum BundlerConfig {
    RemoveUnusedModInLib,
}
