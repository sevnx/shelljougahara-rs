//! The content of a file inode.

#[derive(Debug, Clone)]
pub struct File {
    pub content: String,
}

impl File {
    pub fn new() -> Self {
        Self {
            content: String::new(),
        }
    }
}

impl Default for File {
    fn default() -> Self {
        Self::new()
    }
}
