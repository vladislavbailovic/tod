use crate::comment_type::*;

#[derive(Debug, Default, Clone)]
pub struct Todo {
    pub file: String,
    pub line: usize,
    pub(super) pos: usize,
    pub(super) priority: usize,
    pub(super) todo: String,
}

impl std::fmt::Display for Todo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "TODO: {}:{}:{}\n\t{}",
            self.file, self.line, self.pos, self.todo
        )?;
        Ok(())
    }
}
