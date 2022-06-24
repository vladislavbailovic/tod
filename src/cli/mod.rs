use std::io;

mod args;
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
    let flags: args::Args = Default::default();

    let cmd: Box<dyn Runnable> = if flags.positional.len() > 1 {
        parse_subcommand_options(flags)
    } else {
        Box::new(command_help::Command::default())
    };
    cmd
}

fn parse_subcommand_options(args: args::Args) -> Box<dyn Runnable> {
    match args.positional[1].as_str() {
        "ls" | "list" => parse_path(command_list::Command::default(), args),
        "mark" => {
            if args.positional.len() > 1 {
                let mut cmd = command_mark::Command::new(&args.positional[2]);
                if let Some(cmt) = args.named.get("-c") {
                    cmd.set_comment(cmt);
                }
                if let Some(cmt) = args.named.get("--comment") {
                    cmd.set_comment(cmt);
                }
                parse_path(cmd, args)
            } else {
                Box::new(command_help::Command::default())
            }
        }
        "help" | "--help" | "-h" => Box::new(command_help::Command::default()),
        _ => Box::new(command_help::Command::default()),
    }
}

fn parse_path<T: 'static + WithCwd + Runnable>(mut cmd: T, args: args::Args) -> Box<dyn Runnable> {
    if let Some(path) = args.named.get("-d") {
        cmd.set_cwd(path);
    }
    if let Some(path) = args.named.get("--dir") {
        cmd.set_cwd(path);
    }
    Box::new(cmd)
}
