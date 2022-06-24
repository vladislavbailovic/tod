use std::fs::File;
use std::io::{self, BufRead};

use crate::sources::*;
use crate::todo::*;

pub fn all(path: &str) -> Result<Vec<Todo>, io::Error> {
    let mut todos = Vec::new();
    for src in ls_sources(path)? {
        todos.append(&mut parse_file(&src)?);
    }
    Ok(todos)
}

pub fn find(path: &str, which: &str) -> Result<Todo, io::Error> {
    all(path)?
        .into_iter()
        .find(|x| x.get_id().starts_with(which))
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("ERROR: no todo with ID matching {} in {}", which, path),
            )
        })
}

fn parse_file(path: &str) -> Result<Vec<Todo>, io::Error> {
    let mut todos = Vec::new();
    let file = File::open(path)?;
    for (idx, line) in io::BufReader::new(file).lines().enumerate() {
        let line = line?;
        if let Some(todo) = parser::TodoParser::parse(path, &line, idx) {
            todos.push(todo);
        }
    }
    Ok(todos)
}
