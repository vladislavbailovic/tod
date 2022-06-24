use std::collections::HashMap;

pub(super) struct Args {
    pub(super) named: HashMap<String, String>,
    pub(super) positional: Vec<String>,
}

impl Default for Args {
    fn default() -> Self {
        Self::from(std::env::args().collect::<Vec<String>>())
    }
}

impl From<Vec<String>> for Args {
    fn from(args: Vec<String>) -> Self {
        let mut named = HashMap::new();
        let mut positional = Vec::new();

        let args = Self::normalize(args);
        let mut args = args.iter();
        while let Some(arg) = args.next() {
            if arg.contains('-') {
                let name = arg.clone();
                if let Some(arg) = args.next() {
                    named.insert(name, arg.clone());
                }
            } else {
                positional.push(arg.clone());
            }
        }

        Self { named, positional }
    }
}

impl Args {
    fn normalize(what: Vec<String>) -> Vec<String> {
        let mut result = Vec::new();
        for arg in what {
            if arg.contains('-') && arg.contains('=') {
                if let Some((arg, value)) = arg.split_once('=') {
                    result.push(arg.to_string());
                    result.push(value.to_string());
                }
            } else {
                result.push(arg);
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn normalize_breaks_arg_on_equals() {
        let result = Args::normalize(vec!["-one=test".to_string()]);
        assert!(
            result.len() == 2,
            "unexpected len: {:?} [{}]",
            result,
            result.len()
        );
        assert!(result.contains(&"-one".to_string()));
        assert!(result.contains(&"test".to_string()));
    }

    #[test]
    fn normalize_requires_dashes_to_break() {
        let result = Args::normalize(vec!["one=test".to_string()]);
        assert!(
            result.len() == 1,
            "unexpected len: {:?} [{}]",
            result,
            result.len()
        );
        assert!(result.contains(&"one=test".to_string()));
    }

    #[test]
    fn recognizes_positional_args() {
        let args = Args::from(vec![
            "one".to_string(),
            "two".to_string(),
            "three".to_string(),
        ]);
        assert!(args.positional.len() == 3);
        assert!(args.named.len() == 0);
    }

    #[test]
    fn recognizes_named_args() {
        let args = Args::from(vec![
            "-one=test".to_string(),
            "-two".to_string(),
            "value".to_string(),
        ]);
        assert!(args.positional.len() == 0);
        assert!(args.named.len() == 2);
        assert!(args.named["-one"] == "test");
        assert!(args.named["-two"] == "value");
    }
}

#[cfg(test)]
mod default {
    use super::*;

    #[test]
    fn args_are_loaded_from_env() {
        let _result: Args = Default::default();
    }
}
