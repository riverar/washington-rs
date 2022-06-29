use rayon::prelude::*;
use std::{collections::HashSet, io::prelude::*};
use windows_metadata::reader::{Type, TypeReader, TypeTree};

fn main() {
    let start = std::time::Instant::now();
    let mut output_path = std::path::PathBuf::from("");

    output_path.push("src/Microsoft");
    let _ = std::fs::remove_dir_all(&output_path);
    output_path.pop();

    let reader = TypeReader::get();
    let root = reader.types.get_namespace("Microsoft").unwrap();

    let mut trees = Vec::new();
    collect_subtrees(&output_path, root, &mut trees);

    trees
        .par_iter()
        .for_each(|tree| gen_tree(&output_path, tree));

    output_path.pop();
    output_path.push("Cargo.toml");
    write_toml(&output_path, root);

    println!("Elapsed: {} ms", start.elapsed().as_millis());
}

fn write_toml(output: &std::path::Path, tree: &TypeTree) {
    let mut file = std::fs::File::create(&output).unwrap();

    file.write_all(
        r#"[package]
name = "washington-rs"
version = "0.1.0"
authors = [""]
edition = "2018"
license = "MIT OR Apache-2.0"
description = "Rust for Azure Service Fabric SDK"
repository = ""
documentation = ""
readme = ""
exclude = [".github", ".windows", ".metadata", "docs", "tests"]

[workspace]
members = [
    "crates/tools/*",
    "crates/samples/*",
]

[package.metadata.docs.rs]
default-target = "x86_64-pc-windows-msvc"
targets = []

[dependencies.windows]
version = "0.38"
features = [
    "alloc",
    "Win32_Foundation"
]

[features]
default = []
deprecated = []
implement = ["windows/implement"]
"#
        .as_bytes(),
    )
    .unwrap();

    write_features(&mut file, tree.namespace, tree);
}

fn write_features(
    file: &mut std::fs::File,
    root: &'static str,
    tree: &windows_metadata::reader::TypeTree,
) {
    for tree in tree.namespaces.values() {
        write_feature(file, root, tree);
        write_features(file, root, tree);
    }
}

fn write_feature(
    file: &mut std::fs::File,
    root: &'static str,
    tree: &windows_metadata::reader::TypeTree,
) {
    let reader = TypeReader::get();
    let dependencies = tree
        .types
        .keys()
        .filter_map(|t| {
            let def = reader.get_type((tree.namespace, *t)).unwrap();
            match def {
                Type::TypeDef(def) => Some(
                    def.methods()
                        .flat_map(|m| m.cfg().features("Microsoft"))
                        .collect(),
                ),
                Type::MethodDef(def) => Some(def.cfg().features("Microsoft")),
                _ => None,
            }
        })
        .flatten()
        .collect::<HashSet<&str>>();

    let feature = tree.namespace[root.len() + 1..].replace('.', "_");
    file.write_all(format!("{feature} = [").as_bytes()).unwrap();

    let mut sub_features = Vec::<String>::new();

    if let Some(pos) = feature.rfind('_') {
        let dependency = &feature[..pos];
        sub_features.push(dependency.to_string());
    }

    dependencies
        .iter()
        .filter(|f| f.starts_with("Windows."))
        .map(|f| f[8..].replace('.', "_"))
        .for_each(|f| sub_features.push(format!("windows/{}", f)));

    file.write_all(
        sub_features
            .iter()
            .map(|f| format!("\"{f}\""))
            .collect::<Vec<_>>()
            .join(",")
            .as_bytes(),
    )
    .unwrap();
    file.write_all("]\n".as_bytes()).unwrap();
}

fn collect_subtrees<'a>(
    output: &std::path::Path,
    tree: &'a windows_metadata::reader::TypeTree,
    trees: &mut Vec<&'a windows_metadata::reader::TypeTree>,
) {
    trees.push(tree);

    tree.namespaces
        .values()
        .for_each(|tree| collect_subtrees(output, tree, trees));

    let mut path = std::path::PathBuf::from(output);
    path.push(tree.namespace.replace('.', "/"));
    std::fs::create_dir_all(&path).unwrap();
}

fn gen_tree(output: &std::path::Path, tree: &TypeTree) {
    let mut path = std::path::PathBuf::from(output);

    path.push(tree.namespace.replace('.', "/"));

    let gen = windows_bindgen::Gen {
        namespace: tree.namespace,
        min_xaml: true,
        cfg: false,
        doc: false,
        windows_extern: true,
        ..Default::default()
    };
    let mut tokens = windows_bindgen::gen_namespace(&gen);
    tokens.push_str(r#"#[cfg(feature = "implement")] ::core::include!("impl.rs");"#);
    fmt_tokens(tree.namespace, &mut tokens);
    std::fs::write(path.join("mod.rs"), tokens).unwrap();

    let mut tokens = windows_bindgen::gen_namespace_impl(&gen);
    fmt_tokens(tree.namespace, &mut tokens);
    std::fs::write(path.join("impl.rs"), tokens).unwrap();
}

fn fmt_tokens(namespace: &str, tokens: &mut String) {
    let mut child = std::process::Command::new("rustfmt")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .spawn()
        .expect("Failed to spawn `rustfmt`");

    let mut stdin = child.stdin.take().expect("Failed to open stdin");
    stdin.write_all(tokens.as_bytes()).unwrap();
    drop(stdin);
    let output = child.wait_with_output().unwrap();

    if output.status.success() {
        *tokens = String::from_utf8(output.stdout).expect("Failed to parse UTF-8");
    } else {
        println!("** {} - rustfmt failed", namespace);
    }
}
