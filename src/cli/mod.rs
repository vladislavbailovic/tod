use std::io;

mod command_help;
mod command_list;
mod command_mark;
mod subcommand;

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
    let (subcommand, args) = subcommand::Arguments::subcommand(&args);

    let help = subcommand::Flag {
        name: "help",
        kind: subcommand::FlagType::Boolean,
    };

    let cmd: Box<dyn Runnable> = match subcommand {
        Some("ls") | Some("list") => {
            let supported = [
                help,
                subcommand::Flag {
                    name: "dir",
                    kind: subcommand::FlagType::Value,
                },
                subcommand::Flag {
                    name: "format",
                    kind: subcommand::FlagType::Value,
                },
            ];
            let mut supported = subcommand::Arguments::new(&supported);
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
            let save = subcommand::Flag {
                name: "save",
                kind: subcommand::FlagType::Boolean,
            };
            let supported = [
                help,
                save,
                subcommand::Flag {
                    name: "dir",
                    kind: subcommand::FlagType::Value,
                },
                subcommand::Flag {
                    name: "comment",
                    kind: subcommand::FlagType::Value,
                },
            ];
            let mut supported = subcommand::Arguments::new(&supported);
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

        None | _ => Box::new(command_help::Command::default()),
    };
    cmd
}

// pub fn parse() -> Box<dyn Runnable> {
//     let flags: args::Args = Default::default();

//     let cmd: Box<dyn Runnable> = if flags.positional.len() > 1 {
//         parse_subcommand_options(flags)
//     } else {
//         Box::new(command_help::Command::default())
//     };
//     cmd
// }

// fn parse_subcommand_options(args: args::Args) -> Box<dyn Runnable> {
//     match args.positional[1].as_str() {
//         "ls" | "list" => {
//             let mut cmd = command_list::Command::default();
//             if let Some(fmt) = args.named.get("-f") {
//                 cmd.set_format(fmt);
//             }
//             if let Some(fmt) = args.named.get("--format") {
//                 cmd.set_format(fmt);
//             }
//             parse_path(cmd, args)
//         }
//         "mark" => {
//             if args.positional.len() > 1 {
//                 let mut cmd = command_mark::Command::new(&args.positional[2]);
//                 if let Some(cmt) = args.named.get("-c") {
//                     cmd.set_comment(cmt);
//                 }
//                 if let Some(cmt) = args.named.get("--comment") {
//                     cmd.set_comment(cmt);
//                 }
//                 if let Some(_) = args.named.get("--save") {
//                     cmd.set_save();
//                 }
//                 if let Some(_) = args.named.get("-s") {
//                     cmd.set_save();
//                 }
//                 parse_path(cmd, args)
//             } else {
//                 Box::new(command_help::Command::default())
//             }
//         }
//         "help" | "--help" | "-h" => Box::new(command_help::Command::default()),
//         _ => Box::new(command_help::Command::default()),
//     }
// }

// fn parse_path<T: 'static + WithCwd + Runnable>(mut cmd: T, args: args::Args) -> Box<dyn Runnable> {
//     if let Some(path) = args.named.get("-d") {
//         cmd.set_cwd(path);
//     }
//     if let Some(path) = args.named.get("--dir") {
//         cmd.set_cwd(path);
//     }
//     Box::new(cmd)
// }
