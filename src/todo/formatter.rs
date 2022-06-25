use super::{Format, Todo};
use crate::priority::*;

trait StringFormatter {
    fn format(&self) -> String;
}

pub struct Formatter {
    format: Format,
}
impl Formatter {
    pub fn new(format: Format) -> Self {
        Self { format }
    }

    pub fn format(&self, todo: Todo) -> String {
        match self.format {
            Format::Editor => EditorSpecificFormatter { todo }.format(),
            Format::Default => DefaultFormatter { todo }.format(),
        }
    }
}

struct DefaultFormatter {
    todo: Todo,
}
impl StringFormatter for DefaultFormatter {
    fn format(&self) -> String {
        let todo = "TODO".to_owned()
            + &match self.todo.priority {
                Priority::Normal => "".to_string(),
                Priority::High(d) => format!(" ({})", d),
            };
        format!(
            "[{}] {}: {}:{}:{}\n\t{}",
            self.todo.get_id(),
            todo,
            self.todo.file,
            self.todo.line,
            self.todo.pos,
            self.todo.todo,
        )
    }
}

struct EditorSpecificFormatter {
    todo: Todo,
}
impl StringFormatter for EditorSpecificFormatter {
    fn format(&self) -> String {
        format!(
            "{}:{}:{} \t {} \t {}",
            self.todo.file,
            self.todo.line,
            self.todo.pos,
            self.todo.get_id(),
            self.todo.todo,
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn default_formatter() {
        let todo = Todo {
            file: "test.rs".to_string(),
            line: 1312,
            pos: 161,
            todo: "wow".to_string(),
            ..Default::default()
        };
        let f = Formatter {
            format: Format::Default,
        };

        let out: String = f.format(todo);

        assert_eq!("[d43a44aee6e863c] TODO: test.rs:1312:161\n\twow", out);
    }

    #[test]
    fn editor_formatter() {
        let todo = Todo {
            file: "test.rs".to_string(),
            line: 1312,
            pos: 161,
            todo: "wow".to_string(),
            ..Default::default()
        };
        let f = Formatter {
            format: Format::Editor,
        };

        let out: String = f.format(todo);

        assert_eq!("test.rs:1312:161 \t d43a44aee6e863c \t wow", out);
    }
}
