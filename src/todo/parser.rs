use crate::comment_type::*;
use crate::priority::*;
use crate::todo::Todo;

#[derive(Default)]
pub struct TodoParser {
    pos: usize,
    comment_type: CommentType,
    priority: Priority,
    raw: String,
    todo: String,
}

impl TodoParser {
    pub fn parse(file: &str, raw: &str, line: usize) -> Option<Todo> {
        if !raw.contains("TODO") {
            return None;
        }

        let mut me = Self {
            raw: raw.to_string(),
            ..Default::default()
        };

        me.detect_todo();

        match me.comment_type {
            CommentType::Unknown => None,
            _ => Some(Todo {
                file: file.to_string(),
                line,
                pos: me.pos,
                todo: me.todo,
                priority: me.priority,
            }),
        }
    }

    fn detect_todo(&mut self) {
        if let Some(pos) = self.raw.find("TODO") {
            self.pos = pos + "TODO".len();

            self.detect_todo_string();

            let before: Vec<char> = self.raw.chars().take(pos).collect();
            self.detect_todo_comment_type(before)
        }
    }

    fn detect_todo_comment_type(&mut self, mut before: Vec<char>) {
        let mut is_slash = false;
        let mut is_star = false;
        while let Some(c) = before.pop() {
            match c {
                '#' => {
                    self.comment_type = CommentType::OnelineHash;
                    break;
                }
                '/' if is_slash => {
                    self.comment_type = CommentType::OnelineC;
                    break;
                }
                '*' => {
                    self.comment_type = CommentType::MultilineC;
                    break;
                }
                '/' if is_star => {}
                '/' => {
                    is_slash = true;
                    is_star = false;
                }
                _ => {
                    is_star = false;
                    is_slash = false;
                }
            };
        }
        if self.comment_type == CommentType::Unknown && is_star {
            self.comment_type = CommentType::MultilineC;
        }
    }

    fn detect_todo_string(&mut self) {
        if let Some(priority) = self.consume('!') {
            self.set_priority(priority);
        }
        self.consume(':');
        self.consume(' ');
        self.todo = self.raw.chars().skip(self.pos).collect();
    }

    fn consume(&mut self, what: char) -> Option<usize> {
        let old = self.pos;
        while let Some(c) = self.raw.chars().nth(self.pos) {
            if c == what {
                self.pos += 1;
                continue;
            }
            break;
        }
        if self.pos == old {
            None
        } else {
            Some(self.pos - old)
        }
    }

    fn set_priority(&mut self, priority: usize) {
        self.priority = Priority::from(priority)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn detect_oneline_c() {
        let mut p = TodoParser {
            raw: "// @TODO: this is a string".to_string(),
            ..Default::default()
        };
        p.detect_todo();

        assert!(p.comment_type == CommentType::OnelineC);
        assert!(p.todo == "this is a string");
    }

    #[test]
    fn detect_multiline_c_opening_docblock() {
        let mut p = TodoParser {
            raw: "/** @TODO: this is a string".to_string(),
            ..Default::default()
        };
        p.detect_todo();

        assert!(
            p.comment_type == CommentType::MultilineC,
            "should be multiline c"
        );
        assert!(p.todo == "this is a string");
    }

    #[test]
    fn detect_multiline_c_normal_opening() {
        let mut p = TodoParser {
            raw: "/* @TODO: this is a string".to_string(),
            ..Default::default()
        };
        p.detect_todo();

        assert!(
            p.comment_type == CommentType::MultilineC,
            "should be multiline c"
        );
        assert!(p.todo == "this is a string");
    }

    #[test]
    fn detect_multiline_c_normal_body() {
        let mut p = TodoParser {
            raw: "* TODO!: this is a string".to_string(),
            ..Default::default()
        };
        p.detect_todo();

        assert!(
            p.comment_type == CommentType::MultilineC,
            "should be multiline c"
        );
        assert!(p.todo == "this is a string");
    }
}
