use std::io;

mod command_help;
mod command_list;
mod command_mark;

pub trait Runnable {
    fn run(&self) -> io::Result<()>;
}

pub fn parse() -> Box<dyn Runnable> {
    Box::new(command_mark::Command::default())
}
