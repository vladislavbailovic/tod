use super::*;

include!(concat!(env!("OUT_DIR"), "/help.rs"));

#[derive(Default)]
pub struct Command {}
impl Runnable for Command {
    fn run(&self) -> io::Result<()> {
        println!("{}", get_help_content());
        Ok(())
    }
}
