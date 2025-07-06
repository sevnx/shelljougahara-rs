//! The content of a file inode.

use crate::fs::inode::size::Size;

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

impl Size for File {
    fn size(&self) -> u64 {
        self.content.len() as u64
    }
}

impl Default for File {
    fn default() -> Self {
        Self::new()
    }
}
