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
    args.next();
    let cmd: Box<dyn Runnable> = {
        let arg = args.next();
        if let Some(arg) = arg {
            match arg.as_str() {
                "ls" | "list" => Box::new(command_list::Command::default()),
                "mark" => {
                    let idx = args.next();
                    if let Some(idx) = idx {
                        let idx = idx.parse::<usize>();
                        if let Ok(idx) = idx {
                            let cmd = command_mark::Command::new(idx);
                            return Box::new(cmd);
                        }
                    }
                    return Box::new(command_help::Command::default());
                }
                _ => Box::new(command_help::Command::default()),
            }
        } else {
            Box::new(command_help::Command::default())
        }
    };
    cmd
}
