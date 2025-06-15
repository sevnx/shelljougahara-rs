#![doc = include_str!("../README.md")]

// Crate modules
mod commands;
mod errors;
mod fs;
mod sessions;
mod shell;

// Re-export
pub use errors::ShellError;
pub use fs::{
    FileSystem,
    inode::{Inode, content::InodeContent, metadata::InodeMetadata},
    permissions::FilePermissions,
    users::{Group, GroupId, User, UserId},
};
pub use shell::Shell;
