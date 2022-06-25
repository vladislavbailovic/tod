use super::*;
use crate::actions::scan;
use crate::todo::{Format, Formatter};

pub struct Command {
    path: String,
    format: Format,
}

impl Command {
    pub fn set_format(&mut self, fmt: &str) {
        self.format = fmt.to_string().into();
    }
}

impl Default for Command {
    fn default() -> Self {
        Self {
            path: ".".to_string(),
            format: Format::Default,
        }
    }
}

impl Runnable for Command {
    fn run(&self) -> io::Result<()> {
        let todos = scan::all(&self.path)?;
        let formatter = Formatter::new(self.format);
        for todo in todos {
            println!("{}", formatter.format(todo));
        }
        Ok(())
    }
}

impl WithCwd for Command {
    fn get_cwd(&self) -> String {
        self.path.as_str().to_string()
    }
    fn set_cwd(&mut self, path: &str) {
        self.path = path.to_string();
    }
}
