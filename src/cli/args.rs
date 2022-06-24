use std::collections::HashMap;

#[derive(Debug)]
struct Args {
    pub(super) named: HashMap<String, String>,
    pub(super) positional: Vec<String>,
}

impl From<Vec<String>> for Args {
    fn from(args: Vec<String>) -> Self {
        Self{
            named: HashMap::new(),
            positional: Self::normalize(args)
        }
    }
}

impl Args {
    fn normalize(what: Vec<String>) -> Vec<String> {
        let mut result = Vec::new();
        for arg in what {
            if arg.contains("-") && arg.contains("=") {
                if let Some((arg, value)) = arg.split_once("=") {
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
    fn recognizes_positional_args() {
        let args = Args::from(vec![ "one".to_string(), "two".to_string(), "three".to_string() ]);
        assert!(args.positional.len() == 3);
        assert!(args.named.len() == 0);
    }

    #[test]
    fn normalize_breaks_arg_on_equals() {
        let result = Args::normalize(vec![ "-one=test".to_string() ]);
        assert!(result.len() == 2, "unexpected len: {:?} [{}]", result, result.len());
        assert!(result.contains(&"-one".to_string()));
        assert!(result.contains(&"test".to_string()));
    }
}
