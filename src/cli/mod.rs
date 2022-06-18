use std::env;
use std::io;

mod command_help;
mod command_list;
mod command_mark;

pub trait Runnable {
    fn run(&self) -> io::Result<()>;
}

pub fn parse() -> Box<dyn Runnable> {
    let mut args = env::args();
    let cmd: Box<dyn Runnable>;
    if let Some(subcommand) = args.nth(1) {
        cmd = parse_subcommand_options(&subcommand, args.collect());
    } else {
        cmd = Box::new(command_help::Command::default());
    }
    cmd
}

pub fn parse_subcommand_options(subcmd: &str, args: Vec<String>) -> Box<dyn Runnable> {
    let mut args = args.iter();
    match subcmd {
        "ls" | "list" => Box::new(command_list::Command::default()),
        "mark" => {
            let idx = if let Some(idx) = args.next() {
                idx.parse::<usize>().ok()
            } else {
                None
            };
            if let Some(idx) = idx {
                Box::new(command_mark::Command::new(idx))
            } else {
                Box::new(command_help::Command::default())
            }
        }
        "help" | "--help" | "-h" => Box::new(command_help::Command::default()),
        _ => Box::new(command_help::Command::default()),
    }
}
