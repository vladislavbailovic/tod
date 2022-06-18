use std::fs;
use std::io;
use std::path;

include!(concat!(env!("OUT_DIR"), "/lists.rs"));

pub fn ls_sources(path: &str) -> Result<Vec<String>, io::Error> {
    let mut sources = Vec::new();
    let allow = get_allowlist_extensions();
    for file in ls_files(path)? {
        let path = path::Path::new(&file);
        if let Some(extension) = path.extension() {
            if allow.contains(&extension.to_str().unwrap().to_string()) {
                sources.push(file);
            }
        }
    }
    Ok(sources)
}

fn ls_files(path: &str) -> Result<Vec<String>, io::Error> {
    let path = path::Path::new(path);
    if !path.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("ERROR: {} is not a directory", path.display()),
        ));
    }

    let mut files = Vec::new();
    let blocklist = get_blocklist_directories();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if let Some(last) = path.file_name() {
            let last = last.to_str().unwrap();
            if blocklist.contains(&last.to_string()) {
                continue;
            }
        }

        if let Some(pathstr) = path.canonicalize()?.to_str() {
            if path.is_dir() {
                files.append(&mut ls_files(pathstr)?);
            } else {
                files.push(pathstr.to_string());
            }
        }
    }

    Ok(files)
}
