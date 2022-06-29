use super::*;
use crate::actions::mark::Marker;

#[derive(Debug)]
pub struct Command {
    id: String,
    path: String,
    comment: Option<String>,
    save: bool,
    marker: Option<Marker>
}

impl Command {
    // TODO: clear entire comment when marking

    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            path: ".".to_string(),
            comment: None,
            save: false,
            marker: None,
        }
    }

    pub fn set_comment(&mut self, cmt: &str) {
        self.comment = Some(cmt.to_string());
    }

    pub fn get_comment(&self) -> Option<String> {
        if let Some(comment) = &self.comment {
            return Some(comment.as_str().to_string());
        }
        None
    }

    pub fn set_save(&mut self) {
        self.save = true;
    }

    pub fn set_none(&mut self) {
        self.marker = Some(Marker::None(self.get_comment()));
    }

    pub fn set_done(&mut self) {
        self.marker = Some(Marker::Done(self.get_comment()));
    }
}

impl Runnable for Command {
    fn run(&self) -> io::Result<()> {
        let mut marker = &Marker::Done(self.get_comment());
        if let Some(m) = &self.marker {
            marker = m;
        }
        let replacer = marker.mark(&self.path, &self.id)?;
        let lines = if self.save {
            replacer.replace()?
        } else {
            replacer.dry_run()?
        };

        for (idx, line) in lines.iter().enumerate() {
            if idx == replacer.affected_line() {
                println!("[{:>4}] {}", idx + 1, line);
            } else {
                println!("{:>5}: {}", idx + 1, line);
            }
        }
        Ok(())
    }
}

impl WithCwd for Command {
    fn get_cwd(&self) -> String {
        self.path.as_str().to_string()
    }
    fn set_cwd(&mut self, path: &str) {
        self.path = path.to_string();
    }
}
