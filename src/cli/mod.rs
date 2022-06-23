use std::env;
use std::io;

mod command_help;
mod command_list;
mod command_mark;

pub trait Runnable {
    fn run(&self) -> io::Result<()>;
}

pub trait WithCwd {
    fn get_cwd(&self) -> String;
    fn set_cwd(&mut self, path: &str);
}

pub fn parse() -> Box<dyn Runnable> {
    let mut args = env::args();
    let cmd: Box<dyn Runnable> = if let Some(subcommand) = args.nth(1) {
        parse_subcommand_options(&subcommand, args.collect())
    } else {
        Box::new(command_help::Command::default())
    };
    cmd
}

fn parse_subcommand_options(subcmd: &str, args: Vec<String>) -> Box<dyn Runnable> {
    match subcmd {
        "ls" | "list" => parse_path(command_list::Command::default(), args),
        "mark" => {
            if let Some(idx) = args.get(0) {
                parse_path(command_mark::Command::new(idx), args)
            } else {
                Box::new(command_help::Command::default())
            }
        }
        "help" | "--help" | "-h" => Box::new(command_help::Command::default()),
        _ => Box::new(command_help::Command::default()),
    }
}

fn parse_path<T: 'static + WithCwd + Runnable>(mut cmd: T, args: Vec<String>) -> Box<dyn Runnable> {
    let mut path = None;
    {
        let mut args = args.iter();
        while let Some(arg) = args.next() {
            if "-d" == arg || "--dir" == arg {
                path = args.next();
                break;
            }
        }
    };
    if let Some(path) = path {
        cmd.set_cwd(path);
    }
    Box::new(cmd)
}
