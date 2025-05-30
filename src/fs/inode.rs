//! Inode is a file system object that represents an entry in the file system.

use std::rc::Weak;

use content::InodeContent;
use metadata::InodeMetadata;

pub mod content;
pub mod metadata;

pub struct Inode {
    pub name: String,
    pub content: InodeContent,
    pub metadata: InodeMetadata,
    pub parent: Option<Weak<Inode>>,
}

impl Inode {
    pub fn new(
        name: String,
        content: InodeContent,
        metadata: InodeMetadata,
        parent: Option<Weak<Inode>>,
    ) -> Self {
        Self {
            name,
            content,
            metadata,
            parent,
        }
    }
}
