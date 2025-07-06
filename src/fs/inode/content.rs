//! The content that holds the data of an inode.

mod directory;
mod file;
mod link;

pub use directory::Directory;
pub use file::File;
pub use link::Link;

use crate::fs::inode::size::Size;

#[derive(Debug, Clone)]
#[enum_dispatch::enum_dispatch(Size)]
pub enum InodeContent {
    File(File),
    Directory(Directory),
    Link(Link),
}

impl Size for InodeContent {
    fn size(&self) -> u64 {
        match self {
            InodeContent::File(file) => file.size(),
            InodeContent::Directory(directory) => directory.size(),
            InodeContent::Link(link) => link.size(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InodeType {
    File,
    Directory,
    Link,
}
