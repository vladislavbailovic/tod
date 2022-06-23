use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

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
            todo,
            self.file,
            self.line,
            self.pos,
            self.todo,
        )?;
        Ok(())
    }
}

impl Todo {
    pub fn get_id(&self) -> String {
        format!("{:x}", self.hash())
    }

    fn hash(&self) -> u64 {
        let mut h = DefaultHasher::new();
        let fmt = format!("{}:{}:{}", self.file, self.line, self.pos);
        fmt.hash(&mut h);
        h.finish()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hashing_todos_same_info() {
        let t1 = Todo {
            file: "test".to_string(),
            line: 1312,
            pos: 161,
            priority: Priority::Normal,
            todo: String::from("one"),
        };
        let t2 = Todo {
            file: "test".to_string(),
            line: 1312,
            pos: 161,
            priority: Priority::Normal,
            todo: String::from("two"),
        };
        assert!(t1.get_id() == t2.get_id());
    }
}
