//! Inode is a file system object that represents an entry in the file system.

use content::InodeContent;
use metadata::InodeMetadata;

pub mod content;
pub mod metadata;

pub struct Inode {
    pub name: String,
    pub content: InodeContent,
    pub metadata: InodeMetadata,
}
