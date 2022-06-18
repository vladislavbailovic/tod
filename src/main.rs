use std::fs::{self, File};
use std::io::{self, BufRead};
use std::path;

mod comment_type;
mod todo;

use todo::*;

include!(concat!(env!("OUT_DIR"), "/lists.rs"));

fn main() -> io::Result<()> {
    let files = get_todos("../rssl")?;
    println!("{:#?}\n----------\n", files);
    for file in files {
        println!("- {}", file);
    }
    Ok(())
}

fn get_todos(path: &str) -> Result<Vec<Todo>, io::Error> {
    let mut todos = Vec::new();
    for src in ls_sources(path)? {
        todos.append(&mut parse_file(&src)?);
    }
    Ok(todos)
}

fn parse_file(path: &str) -> Result<Vec<Todo>, io::Error> {
    let mut todos = Vec::new();
    let file = File::open(path)?;
    for (idx, line) in io::BufReader::new(file).lines().enumerate() {
        let line = line?;
        if let Some(todo) = Todo::parse(path, &line, idx) {
            todos.push(todo);
        }
    }
    Ok(todos)
}

fn ls_sources(path: &str) -> Result<Vec<String>, io::Error> {
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

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if let Some(last) = path.file_name() {
            let last = last.to_str().unwrap();
            if last.starts_with('.') {
                continue;
            }
            if last == "node_modules" {
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
