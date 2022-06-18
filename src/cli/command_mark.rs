use super::*;
use crate::actions::mark;

#[derive(Default)]
pub struct Command {
    index: usize,
}

impl Runnable for Command {
    fn run(&self) -> io::Result<()> {
        mark::done(self.index)?;
        Ok(())
    }
}
