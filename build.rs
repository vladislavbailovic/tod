use std::env;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("lists.rs");


    let out = vec![
        format!("fn get_allowlist_extensions() -> Vec<String> {{ vec![{}] }}\n", get_list("config/allowlist/extensions").join(",")),
        format!("fn get_blocklist_directories() -> Vec<String> {{ vec![{}] }}\n", get_list("config/blocklist/directories").join(","))
    ];
    fs::write(&dest_path, out.join("\n")).unwrap();

    println!("cargo:rerun-if-changed=config");
}

fn get_list(path: &str) -> Vec<String> {
    let mut extensions = Vec::new();
    let file = fs::File::open(path).unwrap();
    for line in io::BufReader::new(file).lines() {
        if let Ok(line) = line {
            let mut exts: Vec<String> = line
                .split(',')
                .map(|x| x.trim().to_string())
                .filter(|x| !x.starts_with('#'))
                .collect();
            extensions.append(&mut exts);
        }
    }

    let exts = extensions
        .iter()
        .map(|x| format!("\"{}\".to_string()", x))
        .collect::<Vec<String>>();
    exts
}
