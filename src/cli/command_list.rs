use super::*;
use crate::actions::scan;

#[derive(Default)]
pub struct Command {}
impl Runnable for Command {
    fn run(&self) -> io::Result<()> {
        let todos = scan::get_todos("../rssl")?;
        println!("{:#?}\n----------\n", todos);
        for (idx, todo) in todos.iter().enumerate() {
            println!("- [{:>3}] {}", idx, todo);
        }
        Ok(())
    }
}
