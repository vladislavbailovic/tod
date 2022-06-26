use super::*;
use crate::actions::mark::Marker;

pub struct Command {
    id: String,
    path: String,
    comment: Option<String>,
}

impl Command {
    // TODO: actually save file when marking
    // TODO: clear entire comment when marking

    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            path: ".".to_string(),
            comment: None,
        }
    }

    pub fn set_comment(&mut self, cmt: &str) {
        self.comment = Some(cmt.to_string());
    }
}

impl Runnable for Command {
    fn run(&self) -> io::Result<()> {
        let marker = Marker::Done(&self.comment);
        let replacer = marker.mark(&self.path, &self.id)?;
        let lines = replacer.dry_run()?;
        println!("{:#?}", lines);
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
