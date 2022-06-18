use std::fs::File;
use std::io::{self, BufRead};

use crate::actions::scan;

pub fn done(which: usize) -> io::Result<()> {
    let todos = scan::get_todos("../rssl")?;
    for (idx, todo) in todos.iter().enumerate() {
        if idx != which {
            continue;
        }
        let file = File::open(&todo.file)?;
        let mut lines: Vec<String> = io::BufReader::new(file)
            .lines()
            .filter_map(|x| x.ok())
            .collect();
        lines[todo.line] = lines[todo.line].replace("TODO", "DONE");
        println!("file: {}, lines: \n{:?}", todo.file, lines[todo.line]);
        break;
    }
    Ok(())
}
