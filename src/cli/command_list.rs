use super::*;
use crate::actions::scan;

pub struct Command {
    path: String,
}

impl Default for Command {
    fn default() -> Self {
        Self {
            path: ".".to_string(),
        }
    }
}

impl Runnable for Command {
    fn run(&self) -> io::Result<()> {
        let todos = scan::all(&self.path)?;
        println!("{:#?}\n----------\n", todos);
        for todo in todos {
            println!("- [{:>3}] {}", todo.get_id(), todo);
        }
        Ok(())
    }
}
