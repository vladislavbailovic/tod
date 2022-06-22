use std::io;

mod actions;
mod cli;
mod comment_type;
mod priority;
mod sources;
mod todo;

fn main() -> io::Result<()> {
    let cmd = cli::parse();
    cmd.run()?;
    Ok(())
}
