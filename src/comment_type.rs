#[derive(Debug, PartialEq, Clone)]
pub enum CommentType {
    Unknown,
    OnelineC,
    MultilineC,
    OnelineHash,
}

impl Default for CommentType {
    fn default() -> Self {
        Self::Unknown
    }
}
