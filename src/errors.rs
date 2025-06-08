//! Error types for the shell.

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ShellError {
    #[error("Internal error: {0}")]
    InternalError(String),
    #[error(transparent)]
    FileSystemError(FileSystemError),
}

#[derive(Error, Debug)]
pub enum FileSystemError {
    /// The entry in the file system already exists.
    /// It has been checked in Linux, that even for a directory, it says "File exists"
    #[error("{0}: File exists")]
    EntryAlreadyExists(String),
}
