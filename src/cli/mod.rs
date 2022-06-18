use std::io;

mod command_help;
mod command_list;

pub trait Runnable {
    fn run(&self) -> io::Result<()>;
}

pub fn parse() -> Box<dyn Runnable> {
    Box::new(command_list::Command{})
}
