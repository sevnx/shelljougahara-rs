//! Inode is a file system object that represents an entry in the file system.

use std::{path::PathBuf, rc::Weak};

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

    pub fn path(&self) -> Result<PathBuf, String> {
        let mut path = PathBuf::new();
        if let Some(parent) = self.parent.as_ref() {
            let parent_path = parent.upgrade().ok_or("Parent directory does not exist")?;
            path.push(parent_path.path()?);
        } else {
            path.push("/");
        }

        Ok(path)
    }
}
