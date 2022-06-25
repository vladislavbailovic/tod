#[derive(PartialEq)]
pub enum Format {
    Default,
    Editor,
}

impl Default for Format {
    fn default() -> Self {
        Self::Default
    }
}

impl From<String> for Format {
    fn from(what: String) -> Format {
        match what.as_str() {
            "editor" => Format::Editor,
            _ => Format::Default,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn default_format() {
        let fmt: Format = Default::default();
        assert!(Format::Default == fmt);
    }

    #[test]
    fn from_editor() {
        let fmt = Format::from("editor".to_string());
        assert!(Format::Editor == fmt);
    }

    #[test]
    fn from_whatever_else() {
        let fmt = Format::from("whatever man".to_string());
        assert!(Format::Default == fmt);
    }
}
