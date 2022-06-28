use std::io;

mod command_help;
mod command_list;
mod command_mark;
mod flags;

pub trait Runnable {
    fn run(&self) -> io::Result<()>;
}

pub trait WithCwd {
    fn get_cwd(&self) -> String;
    fn set_cwd(&mut self, path: &str);
}

pub fn parse() -> Box<dyn Runnable> {
    let args: Vec<_> = std::env::args().collect();
    let args: Vec<_> = args.iter().map(|x| x.as_str()).collect();
    let (subcommand, args) = flags::Arguments::subcommand(&args);

    let help = flags::Flag {
        name: "help",
        kind: flags::FlagType::Boolean,
    };

    let cmd: Box<dyn Runnable> = match subcommand {
        Some("ls") | Some("list") => {
            let supported = [
                help,
                flags::Flag {
                    name: "dir",
                    kind: flags::FlagType::Value,
                },
                flags::Flag {
                    name: "format",
                    kind: flags::FlagType::Value,
                },
            ];
            let mut supported = flags::Arguments::new(&supported);
            supported.parse(args);

            if supported.has(help) {
                return Box::new(command_help::Command::default());
            }

            let mut cmd = command_list::Command::default();
            if let Some(dir) = supported.named.get("dir") {
                cmd.set_cwd(dir);
            }
            if let Some(fmt) = supported.named.get("format") {
                cmd.set_format(fmt);
            }
            Box::new(cmd)
        }

        Some("mark") => {
            let save = flags::Flag {
                name: "save",
                kind: flags::FlagType::Boolean,
            };
            let supported = [
                help,
                save,
                flags::Flag {
                    name: "dir",
                    kind: flags::FlagType::Value,
                },
                flags::Flag {
                    name: "comment",
                    kind: flags::FlagType::Value,
                },
            ];
            let mut supported = flags::Arguments::new(&supported);
            supported.parse(args);

            if supported.has(help) {
                return Box::new(command_help::Command::default());
            }

            let mut cmd = command_mark::Command::new(supported.positional[0]);
            if let Some(dir) = supported.named.get("dir") {
                cmd.set_cwd(dir);
            }
            if let Some(cmt) = supported.named.get("comment") {
                cmd.set_comment(cmt);
            }
            if supported.has(save) {
                cmd.set_save();
            }
            Box::new(cmd)
        }

        _ => Box::new(command_help::Command::default()),
    };
    cmd
}
