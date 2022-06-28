use std::collections::HashMap;

#[derive(Clone, Copy)]
struct Flag<'cmd> {
    name: &'cmd str,
    kind: FlagType,
}

impl<'cmd> Flag<'cmd> {
    fn flag_base(flag: &str) -> Option<&str> {
        if flag.contains('-') {
            Some(str::trim_start_matches(flag, '-'))
        } else {
            None
        }
    }

    fn full(&self) -> String {
        format!("--{}", self.name)
    }

    fn short(&self) -> String {
        format!("-{}", &self.name[0..1])
    }
}

#[derive(Clone, Copy, PartialEq)]
enum FlagType {
    Exact,
    Boolean,
    Value,
}

struct Arguments<'cmd> {
    positional: Vec<&'cmd str>,
    named: HashMap<&'cmd str, &'cmd str>,
    boolean: Vec<&'cmd str>,
    exact: Vec<&'cmd str>,
    args: &'cmd[&'cmd str],
    supported: &'cmd [Flag<'cmd>],
}

impl<'cmd> Arguments<'cmd> {
    pub fn new(supported: &'cmd [Flag<'cmd>]) -> Self {
        Self {
            supported,
            args: &[],
            positional: Vec::new(),
            boolean: Vec::new(),
            exact: Vec::new(),
            named: HashMap::new(),
        }
    }

    pub fn parse(&mut self, args: &'cmd[&'cmd str]) {
        self.args = args.clone();
        let args = self.parse_boolean(args);
        let args = self.parse_named(&args);
        let args = self.parse_exact(&args);
        self.positional = args;
    }

    pub fn subcommand(args: &'cmd[&'cmd str]) -> (Option<&'cmd str>, &'cmd[&'cmd str]) {
        if args.len() < 2 {
            return (None, args);
        }
        (Some(args[1]), &args[2..])
    }

    pub fn has(&self, flag: Flag) -> bool {
        match flag.kind {
            FlagType::Boolean => self.boolean.contains(&flag.name),
            FlagType::Exact => self.exact.contains(&flag.name),
            FlagType::Value => {
                if let Some(_) = self.named.get(flag.name) {
                    true
                } else {
                    false
                }
            }
        }
    }

    fn get_supported(&self, kind: FlagType) -> Vec<&'cmd str> {
        self.supported
            .clone()
            .iter()
            .filter_map(|x| if x.kind == kind { Some(x.name) } else { None })
            .collect()
    }

    fn parse_boolean(&mut self, args: &[&'cmd str]) -> Vec<&'cmd str> {
        let boolean: Vec<&'cmd str> = self.get_supported(FlagType::Boolean);
        args.into_iter()
            .filter_map(|&x| {
                if let Some(x) = Flag::flag_base(x) {
                    if boolean.contains(&x) {
                        self.boolean.push(x);
                        return None;
                    }
                }
                Some(x)
            })
            .collect()
    }

    fn parse_named(&mut self, args: &[&'cmd str]) -> Vec<&'cmd str> {
        let named: Vec<&'cmd str> = self.get_supported(FlagType::Value);
        let mut remaining = Vec::new();
        let mut args = args.into_iter();
        while let Some(&arg) = args.next() {
            if let Some(arg) = Flag::flag_base(arg) {
                if named.contains(&arg) {
                    if let Some(value) = args.next() {
                        self.named.insert(arg, value);
                        continue;
                    }
                }
            } else {
                remaining.push(arg);
            }
        }
        remaining
    }

    fn parse_exact(&mut self, args: &[&'cmd str]) -> Vec<&'cmd str> {
        let exact: Vec<&'cmd str> = self.get_supported(FlagType::Exact);
        args.into_iter()
            .filter_map(|&x| {
                if exact.contains(&x) {
                    self.exact.push(x);
                    return None;
                }
                Some(x)
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn supported() {
        let args = Arguments::new(&[Flag {
            name: "help",
            kind: FlagType::Boolean,
        }]);
        let supported = args.get_supported(FlagType::Boolean);
        assert_eq!(supported.len(), 1);
    }

    #[test]
    fn boolean() {
        let mut args = Arguments::new(&[Flag {
            name: "help",
            kind: FlagType::Boolean,
        }]);
        let remaining = args.parse_boolean(&["one", "--help", "two"]);
        assert_eq!(args.boolean.len(), 1);
        assert!(args.has(Flag {
            name: "help",
            kind: FlagType::Boolean
        }));

        assert_eq!(remaining.len(), 2);
    }

    #[test]
    fn named() {
        let mut args = Arguments::new(&[Flag {
            name: "one",
            kind: FlagType::Value,
        }]);
        let remaining = args.parse_named(&["--one", "two", "three"]);
        assert_eq!(args.named.len(), 1);
        assert_eq!(remaining.len(), 1);

        assert_eq!(args.named["one"], "two");
    }

    #[test]
    fn exact() {
        let mut args = Arguments::new(&[Flag {
            name: "help",
            kind: FlagType::Exact,
        }]);
        let remaining = args.parse_exact(&["one", "help", "--help", "two"]);
        assert_eq!(args.exact.len(), 1);
        assert!(args.has(Flag {
            name: "help",
            kind: FlagType::Exact
        }));

        assert_eq!(remaining.len(), 3);
        assert!(remaining.contains(&"--help"));
    }

    #[test]
    fn subcommand_empty() {
        let (subcommand, args) = Arguments::subcommand(&[]);
        match subcommand {
            Some(_) => assert!(false, "there should be no subcommand"),
            None => assert!(true),
        }
        assert_eq!(args.len(), 0);
    }

    #[test]
    fn subcommand_one() {
        let (subcommand, args) = Arguments::subcommand(&["wat"]);

        match subcommand {
            Some(_) => assert!(false, "main command is not subcommand"),
            None => assert!(true),
        }
        assert_eq!(args.len(), 1);
    }

    #[test]
    fn subcommand_not_empty() {
        let (subcommand, args) = Arguments::subcommand(&["tod", "test"]);

        match subcommand {
            Some("test") => assert!(true),
            Some(_) => assert!(false, "wrong subcommand"),
            None => assert!(false, "there should be a subcommand"),
        }
        assert_eq!(args.len(), 0);
    }

    #[test]
    fn usage_ls() {
        let env = vec!["tod", "ls", "--dir", "../wat"];
        let (subcommand, args) = Arguments::subcommand(&env);

        if let Some("ls") = subcommand {
            let mut supported = Arguments::new(&[
                Flag {
                    name: "help",
                    kind: FlagType::Boolean,
                },
                Flag {
                    name: "dir",
                    kind: FlagType::Value,
                },
            ]);
            supported.parse(args);
            if let Some(&"../wat") = supported.named.get("dir") {
                assert!(true);
            } else {
                assert!(false, "named flag not recognized");
            }
        } else {
            assert!(false, "failed parsing subcommand");
        }
    }

    #[test]
    fn test_flag() {
        let f = Flag {
            name: "help",
            kind: FlagType::Boolean,
        };

        assert_eq!("--help", &f.full());
        assert_eq!("-h", &f.short());

        if let Some(flag) = Flag::flag_base("--help") {
            assert_eq!("help", flag);
        }
        if let Some(flag) = Flag::flag_base("-h") {
            assert_eq!("h", flag);
        }
    }
}
