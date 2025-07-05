//! Error types for the shell.

use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ShellError {
    #[error("Internal error: {0}")]
    Internal(String),
    #[error(transparent)]
    FileSystem(FileSystemError),
    #[error(transparent)]
    Session(SessionError),
    #[error("Shell is not active")]
    ShellNotActive,
}

#[derive(Error, Debug, PartialEq)]
pub enum FileSystemError {
    /// The entry in the file system already exists.
    /// It has been checked in Linux, that even for a directory, it says "File exists"
    #[error("{0}: File exists")]
    EntryAlreadyExists(String),
    #[error("'{0}': Not a directory")]
    NotADirectory(String),
    #[error("User already exists: {0}")]
    UserAlreadyExists(String),
    #[error("Directory not found")]
    DirectoryNotFound(String),
    #[error("Incorrect path")]
    IncorrectPath,
    #[error("Entry not found: {0}")]
    EntryNotFound(String),
    #[error("Failed to get parent directory")]
    FailedToGetParent,
}

#[derive(Error, Debug, PartialEq)]
pub enum SessionError {
    #[error("User not found")]
    UserNotFound,
    #[error("No previous working directory")]
    NoPreviousWorkingDirectory,
    #[error("Previous working directory does not exist")]
    PreviousWorkingDirectoryDoesNotExist,
}
