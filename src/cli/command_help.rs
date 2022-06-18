use super::*;

pub struct Command {}
impl Runnable for Command {
    fn run(&self) -> io::Result<()> {
        println!("HALP!");
        Ok(())
    }
}

