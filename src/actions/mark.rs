use std::fs::File;
use std::io::{self, BufRead, Write};

use crate::actions::scan;
use crate::todo::Todo;

// TODO: clean up the whole command/marker API
// TODO: remove entire comment line (if possible) when scrubbing todo

#[derive(Debug)]
pub enum Marker {
    None(Option<String>),
    Done(Option<String>),
}

impl Marker {
    pub fn mark(&self, path: &str, todo_id: &str) -> io::Result<Replacer> {
        let todo = scan::find(path, todo_id)?;
        Ok(Replacer::new(todo, self.get_mark()))
    }

    fn get_mark(&self) -> Option<String> {
        match &self {
            Marker::None(None) => None,
            Marker::None(Some(comment)) => Some(comment.to_string()),
            Marker::Done(Some(comment)) => Some(format!("Done ({}):", comment)),
            Marker::Done(None) => Some(String::from("Done:")),
        }
    }
}

pub struct Replacer {
    todo: Todo,
    mark: Option<String>,
}

impl Replacer {
    fn new(todo: Todo, mark: Option<String>) -> Self {
        Self { todo, mark }
    }

    pub fn dry_run(&self) -> io::Result<Vec<String>> {
        let file = File::open(&self.todo.file)?;
        let lines = io::BufReader::new(file).lines().filter_map(|x| x.ok());
        let lines = self.process_lines(lines);
        Ok(lines)
    }

    pub fn replace(&self) -> io::Result<Vec<String>> {
        let lines = self.dry_run()?;
        let mut file = File::create(&self.todo.file)?;
        write!(file, "{}", lines.join("\n"))?;
        Ok(lines)
    }

    pub fn affected_line(&self) -> usize {
        self.todo.line
    }

    fn process_lines(&self, lines: impl Iterator<Item = String>) -> Vec<String> {
        lines
            .enumerate()
            .map(|(idx, mut line)| {
                if idx != self.todo.line {
                    return line;
                }
                let left = {
                    let (left, _) = line.split_at_mut(self.todo.pos);
                    let left = left.strip_suffix(' ').unwrap_or(left);
                    let left = left.strip_suffix(':').unwrap_or(left);
                    let left = left.strip_suffix('!').unwrap_or(left);

                    let left = &left[..left.len() - 4];

                    left.strip_suffix('@').unwrap_or(left)
                };
                if let Some(mark) = &self.mark {
                    format!("{}{} {}", left, mark, self.todo.get())
                } else {
                    left.to_string()
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn done_with_comment_mark_getting() {
        let comment = Some(String::from("fixed in <hash>"));
        let m = Marker::Done(comment);
        if let Some(mark) = m.get_mark() {
            assert_eq!("Done (fixed in <hash>):", mark);
        } else {
            assert!(false, "expected a string in marker");
        }
    }

    #[test]
    fn mark_returns_replacer() {
        let marker = Marker::None(None);
        let rp = marker.mark(".", "79b");
        if let Ok(rp) = rp {
            if let Some(_) = rp.mark {
                assert!(false, "expected no mark");
            } else {
                assert!(true);
            }
        } else {
            assert!(false, "unable to find a todo from tests");
        }
    }

    #[test]
    fn mark_none_with_string_replaces_todo() {
        let comment = Some(String::from("fixed in <hash>"));
        let m = Marker::None(comment);
        if let Some(mark) = m.get_mark() {
            assert_eq!("fixed in <hash>", mark);
        } else {
            assert!(false, "expected comment");
        }
    }

    #[test]
    fn replacer_dry_run() {
        let marker = Marker::Done(None);
        let rp = marker.mark(".", "79b").unwrap();
        if let Ok(lines) = rp.dry_run() {
            assert!(lines.len() > 42);
            assert!(lines[rp.todo.line].contains("Done:"));
            assert!(!lines[rp.todo.line].contains("TODO:"));
        } else {
            assert!(false, "expected to be able to replace a line in test");
        }
    }
}
