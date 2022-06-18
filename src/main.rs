use std::io;

mod actions;
mod comment_type;
mod sources;
mod todo;
mod cli;

use actions::*;

fn main() -> io::Result<()> {
    let cmd = cli::parse();
    cmd.run()?;
    Ok(())
}

fn main3() -> io::Result<()> {
    mark::done(1)?;
    Ok(())
}
