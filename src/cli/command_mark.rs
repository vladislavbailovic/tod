use super::*;
use crate::actions::mark;

pub struct Command {
    index: usize,
    path: String,
}

impl Command {
    pub fn new(index: usize) -> Self {
        Self {
            index,
            path: ".".to_string(),
        }
    }
}

impl Runnable for Command {
    fn run(&self) -> io::Result<()> {
        mark::done(&self.path, self.index)?;
        Ok(())
    }
}
