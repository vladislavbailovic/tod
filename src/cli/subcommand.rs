use std::collections::HashMap;

struct Subcommand<'cmd> {
    name: &'cmd str,
    args: Arguments<'cmd>,
}

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

#[derive(Clone, Copy)]
enum FlagType {
    Positional,
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
    }

    fn get_supported(&self, kind: FlagType) -> Vec<&'cmd str> {
         self.supported.clone().iter().filter_map(|x| {
            match x.kind {
                kind => Some(x.name),
                _ => None,
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
        // assert!(args.boolean.contains("--help"));

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
    }
}
