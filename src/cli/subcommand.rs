use std::collections::HashMap;

struct Arguments<'cmd> {
    positional: Vec<&'cmd str>,
    named: HashMap<&'cmd str, &'cmd str>,
    boolean: Vec<&'cmd str>,
    args: Vec<&'cmd str>,
    supported: Vec<Flag<'cmd>>,
}

#[derive(Clone, Copy)]
struct Flag<'cmd> {
    name: &'cmd str,
    kind: FlagType,
}

impl<'cmd> Flag<'cmd> {
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
    Value
}

impl<'cmd> Arguments<'cmd> {
    fn new(supported: Vec<Flag<'cmd>>) -> Self {
        Self {
            supported,
            args: Vec::new(),
            positional: Vec::new(),
            boolean: Vec::new(),
            named: HashMap::new(),
        }
    }

    fn parse(&mut self, args: Vec<&'cmd str>) {
        self.args = args.clone();
        let args = self.parse_boolean(args);
        let args = self.parse_named(args);
        self.positional = args;
    }

    fn subcommand(args: Vec<&'cmd str>) -> (Option<&'cmd str>, Vec<&'cmd str>) {
        if args.len() < 2 {
            return (None, args);
        }
        (Some(args[1]), args[2..].to_vec())
    }

    fn get_supported(&self, kind: FlagType) -> Vec<&'cmd str> {
         self.supported.clone().iter().filter_map(|x| {
             if x.kind == kind {
                 Some(x.name)
             } else {
                 None
             }
        }).collect()
    }

    fn parse_boolean(&mut self, args: Vec<&'cmd str>) -> Vec<&'cmd str> {
        let supported: Vec<&'cmd str> = self.get_supported(FlagType::Boolean);
        args.into_iter().filter(|x| {
            if supported.contains(x) {
                self.boolean.push(x);
                false
            } else {
                true
            }
        }).collect()
    }

    fn parse_named(&mut self, args: Vec<&'cmd str>) -> Vec<&'cmd str> {
        let named: Vec<&'cmd str> = self.get_supported(FlagType::Value);
        let mut remaining = Vec::new();
        let mut args = args.iter();
        while let Some(&arg) = args.next() {
            if named.contains(&arg) {
                if let Some(value) = args.next() {
                    self.named.insert(arg, value);
                    continue;
                }
            }
            remaining.push(arg);
        }
        remaining
    }

    fn has(&self, flag: Flag) -> bool {
        match flag.kind {
            FlagType::Boolean => self.boolean.contains(&flag.name),
            FlagType::Value => {
                if let Some(_) = self.named.get(flag.name) {
                    true
                } else {
                    false
                }
            },
            FlagType::Exact => self.positional.len() > 0,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn supported() {
        let args = Arguments::new(vec![
            Flag{name: "--help", kind: FlagType::Boolean}
        ]);
        let supported = args.get_supported(FlagType::Boolean);
        assert_eq!(supported.len(), 1);
    }

    #[test]
    fn boolean() {
        let mut args = Arguments::new(vec![
            Flag{name: "--help", kind: FlagType::Boolean}
        ]);
        let remaining = args.parse_boolean(vec!["one", "--help", "two"]);
        assert_eq!(args.boolean.len(), 1);
        assert!(args.has(Flag{ name: "--help", kind: FlagType::Boolean}));

        assert_eq!(remaining.len(), 2);
    }

    #[test]
    fn named() {
        let mut args = Arguments::new(vec![
            Flag{name: "--one", kind: FlagType::Value}
        ]);
        let remaining = args.parse_named(vec!["--one", "two", "three"]);
        assert_eq!(args.named.len(), 1);
        assert_eq!(remaining.len(), 1);

        assert_eq!(args.named["--one"], "two");
    }

    #[test]
    fn subcommand_empty() {
        let (subcommand, args) = Arguments::subcommand(Vec::new());
        match subcommand {
            Some(_) => assert!(false, "there should be no subcommand"),
            None => assert!(true),
        }
        assert_eq!(args.len(), 0);
    }

    #[test]
    fn subcommand_one() {
        let (subcommand, args) = Arguments::subcommand(vec!["wat"]);

        match subcommand {
            Some(_) => assert!(false, "main command is not subcommand"),
            None => assert!(true),
        }
        assert_eq!(args.len(), 1);
    }

    #[test]
    fn subcommand_not_empty() {
        let (subcommand, args) = Arguments::subcommand(vec!["tod", "test"]);

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
        let (subcommand, args) = Arguments::subcommand(env);

        if let Some("ls") = subcommand {
            let mut supported = Arguments::new(vec![
              Flag{ name: "--help", kind: FlagType::Boolean },
              Flag{ name: "--dir", kind: FlagType::Value },
            ]);
            supported.parse(args);
            if let Some(&"../wat") = supported.named.get("--dir") {
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
        let f = Flag{name: "help", kind: FlagType::Boolean};

        assert_eq!("--help", &f.full());
        assert_eq!("-h", &f.short());
    }
}
