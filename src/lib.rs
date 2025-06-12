//! A simulated shell written in Rust, made for educational purposes, first and foremost in the
//! context of the [DesCode](https://github.com/desforgehub/DesCode) project.

// Crate modules
mod commands;
mod errors;
mod fs;
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
