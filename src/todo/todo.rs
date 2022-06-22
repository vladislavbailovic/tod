use crate::comment_type::*;
use crate::priority::*;

#[derive(Debug, Default, Clone)]
pub struct Todo {
    pub file: String,
    pub line: usize,
    pub(super) pos: usize,
    pub(super) priority: Priority,
    pub(super) todo: String,
}

impl std::fmt::Display for Todo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let todo = "TODO".to_owned()
            + &match self.priority {
                Priority::Normal => "".to_string(),
                Priority::High(d) => format!(" ({})", d),
            };
        write!(
            f,
            "{}: {}:{}:{}\n\t{}",
            todo, self.file, self.line, self.pos, self.todo
        )?;
        Ok(())
    }
}
