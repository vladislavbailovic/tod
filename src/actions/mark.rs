use std::fs::File;
use std::io::{self, BufRead};

use crate::actions::scan;

pub fn done(path: &str, which: &str) -> io::Result<()> {
    let todo = scan::find(path, which)?;

    let file = File::open(&todo.file)?;
    let mut lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .filter_map(|x| x.ok())
        .collect();
    lines[todo.line] = lines[todo.line].replace("TODO", "DONE");
    println!("file: {}, lines: \n{:?}", todo.file, lines[todo.line]);
    println!("{}", todo);

    Ok(())
}
