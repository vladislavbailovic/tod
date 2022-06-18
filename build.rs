use std::env;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    write_lists();
    write_help();

    println!("cargo:rerun-if-changed=config");
}

fn write_help() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("help.rs");

    let file = fs::File::open("config/help").unwrap();
    let out: Vec<String> = io::BufReader::new(file).lines().flatten().collect();
    fs::write(
        &dest_path,
        format!(
            "fn get_help_content() -> String {{ \"{}\".to_string() }}",
            out.join("\n")
        ),
    )
    .unwrap();
}

fn write_lists() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("lists.rs");

    let out = vec![
        format!(
            "fn get_allowlist_extensions() -> Vec<String> {{ vec![{}] }}\n",
            get_list("config/allowlist/extensions").join(",")
        ),
        format!(
            "fn get_blocklist_directories() -> Vec<String> {{ vec![{}] }}\n",
            get_list("config/blocklist/directories").join(",")
        ),
    ];
    fs::write(&dest_path, out.join("\n")).unwrap();
}

fn get_list(path: &str) -> Vec<String> {
    let mut extensions = Vec::new();
    let file = fs::File::open(path).unwrap();
    for line in io::BufReader::new(file).lines().flatten() {
        let mut exts: Vec<String> = line
            .split(',')
            .map(|x| x.trim().to_string())
            .filter(|x| !x.starts_with('#'))
            .collect();
        extensions.append(&mut exts);
    }

    let exts = extensions
        .iter()
        .map(|x| format!("\"{}\".to_string()", x))
        .collect::<Vec<String>>();
    exts
}
