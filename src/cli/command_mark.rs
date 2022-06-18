use super::*;
use crate::actions::mark;

#[derive(Default)]
pub struct Command {
    index: usize,
}

impl Command {
    pub fn new(index: usize) -> Self {
        Self { index }
    }
}

impl Runnable for Command {
    fn run(&self) -> io::Result<()> {
        mark::done(self.index)?;
        Ok(())
    }
}
