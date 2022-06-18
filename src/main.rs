use std::io;

mod actions;
mod comment_type;
mod sources;
mod todo;

use actions::*;

fn main() -> io::Result<()> {
    mark::done(1)?;
    Ok(())
}

fn main2() -> io::Result<()> {
    let todos = scan::get_todos("../rssl")?;
    println!("{:#?}\n----------\n", todos);
    for (idx, todo) in todos.iter().enumerate() {
        println!("- [{:>3}] {}", idx, todo);
    }
    Ok(())
}
