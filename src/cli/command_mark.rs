use super::*;
use crate::actions::mark;

pub struct Command {
    id: String,
    path: String,
    comment: Option<String>,
}

impl Command {
    // TODO: actually save file when marking
    // TODO: add comment support
    // TODO: clear entire comment when marking

    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            path: ".".to_string(),
            comment: None,
        }
    }

    pub fn set_comment(&mut self, cmt: &str) {
        self.comment = Some(cmt.to_string());
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
