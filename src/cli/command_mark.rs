use super::*;
use crate::actions::mark;

pub struct Command {
    id: String,
    path: String,
}

impl Command {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            path: ".".to_string(),
        }
    }
}

impl Runnable for Command {
    fn run(&self) -> io::Result<()> {
        mark::done(&self.path, &self.id)?;
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
