//! The content of a file inode.

pub struct File {
    content: String,
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
