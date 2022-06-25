use std::fs::File;
use std::io::{self, BufRead};

use crate::actions::scan;

pub fn done(path: &str, which: &str, comment: &Option<String>) -> io::Result<()> {
    let todo = scan::find(path, which)?;

    let file = File::open(&todo.file)?;
    let mut lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .filter_map(|x| x.ok())
        .collect();
    let with = if let Some(comment) = comment {
        format!("Done ({})", comment)
    } else {
        "Done".to_string()
    };
    lines[todo.line] = lines[todo.line].replace("TODO", &with);
    println!("file: {}, lines: \n{:?}", todo.file, lines[todo.line]);

    Ok(())
}
