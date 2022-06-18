#[derive(Debug, PartialEq)]
pub enum CommentType {
    Unknown,
    OnelineC,
    MultilineC,
    OnelineHash,
}

impl Default for CommentType {
    fn default() -> Self {
        return Self::Unknown;
    }
}
