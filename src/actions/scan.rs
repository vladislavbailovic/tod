use std::fs::File;
use std::io::{self, BufRead};

use crate::sources::*;
use crate::todo::*;

pub fn get_todos(path: &str) -> Result<Vec<Todo>, io::Error> {
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
