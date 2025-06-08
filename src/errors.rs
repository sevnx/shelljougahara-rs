//! Error types for the shell.

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ShellError {
    #[error("Internal error: {0}")]
    Internal(String),
    #[error(transparent)]
    FileSystem(FileSystemError),
    #[error(transparent)]
    User(UserError),
}

#[derive(Error, Debug)]
pub enum FileSystemError {
    /// The entry in the file system already exists.
    /// It has been checked in Linux, that even for a directory, it says "File exists"
    #[error("{0}: File exists")]
    EntryAlreadyExists(String),
    #[error("Directory not found")]
    DirectoryNotFound(String),
    #[error("Incorrect path")]
    IncorrectPath,
    #[error("Entry not found: {0}")]
    EntryNotFound(String),
}

#[derive(Error, Debug)]
pub enum UserError {
    #[error("User not found")]
    UserNotFound,
}
