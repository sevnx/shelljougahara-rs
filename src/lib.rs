//! A shell written in Rust, made to be used in a WASM environment.

// Crate modules
mod commands;
mod errors;
mod fs;
mod parser;
mod shell;

// Re-export
pub use fs::{
    FileSystem,
    inode::{Inode, content::InodeContent, metadata::InodeMetadata},
    permissions::FilePermissions,
    users::{Group, GroupId, User, UserId},
};
pub use shell::Shell;
