use std::io;

use std::fs::File;
use std::io::BufRead;

mod actions;
mod comment_type;
mod sources;
mod todo;

use actions::*;

fn main() -> io::Result<()> {
    let todos = scan::get_todos("../rssl")?;
    for todo in todos {
        let file = File::open(&todo.file)?;
        let mut lines: Vec<String> = io::BufReader::new(file)
            .lines()
            .filter_map(|x| x.ok())
            .collect();
        lines[todo.line] = lines[todo.line].replace("TODO", "DONE");
        println!("file: {}, lines: \n{:?}", todo.file, lines[todo.line]);
    }
    Ok(())
}

fn main2() -> io::Result<()> {
    let files = scan::get_todos("../rssl")?;
    println!("{:#?}\n----------\n", files);
    for file in files {
        println!("- {}", file);
    }
    Ok(())
}
