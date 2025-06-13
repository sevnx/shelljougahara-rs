//! The content that holds the data of an inode.

mod directory;
mod file;
mod link;

pub use directory::Directory;
pub use file::File;
pub use link::Link;

#[derive(Debug, Clone)]
pub enum InodeContent {
    File(File),
    Directory(Directory),
    Link(Link),
}
