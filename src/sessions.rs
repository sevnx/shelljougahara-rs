//! Representation of a shell session.
//!
//! A session represents a shell instance, a single file system can have multiple sessions.
//! It contains a reference to the file system, and information like the current user and directory.

use std::path::PathBuf;

use crate::{
    FileSystem, ShellError, UserId,
    errors::{FileSystemError, SessionError},
};

#[derive(Debug, Clone)]
pub struct Session {
    current_working_directory: PathBuf,
    previous_working_directory: Option<PathBuf>,
    current_user: UserId,
    executed_commands: Vec<String>,
}

impl Session {
    pub fn new(current_working_directory: PathBuf, current_user: UserId) -> Self {
        Self {
            current_working_directory,
            previous_working_directory: None,
            current_user,
            executed_commands: Vec::new(),
        }
    }

    pub fn change_directory(&mut self, fs: &FileSystem, path: &str) -> Result<(), ShellError> {
        let directory_change = self.parse_directory_change(fs, path)?;
        let prev_working_directory = self.current_working_directory.clone();
        match directory_change {
            DirectoryChange::Path(path) => {
                let inode = fs.find_absolute_inode(&path).ok_or_else(|| {
                    ShellError::FileSystem(FileSystemError::DirectoryNotFound(path.to_string()))
                })?;

                self.current_working_directory =
                    inode.lock().expect("Failed to lock inode").path()?;
            }
            DirectoryChange::Parent => {
                let current =
                    fs.find_absolute_inode(&self.current_working_directory.display().to_string());
                match current {
                    Some(current) => {
                        let current = current.lock().expect("Failed to lock inode");
                        match current.parent() {
                            Some(parent) => {
                                self.current_working_directory = parent
                                    .upgrade()
                                    .expect("Failed to get parent")
                                    .lock()
                                    .expect("Failed to lock inode")
                                    .path()?;
                            }
                            None => {
                                // This could happen if the current working directory is the root
                            }
                        }
                    }
                    None => {
                        // The current working directory does not exist anymore, we try to get
                        // parent by removing the last component
                        let mut path = self.current_working_directory.clone();
                        path.pop();
                        let current = fs.find_absolute_inode(&path.display().to_string());
                        match current {
                            Some(current) => {
                                self.current_working_directory =
                                    current.lock().expect("Failed to lock inode").path()?;
                            }
                            None => {
                                // Even the parent does not exist, tried with Bash and it errored
                                // cd: error retrieving current directory:
                                // getcwd: cannot access parent directories: No such file or directory
                                return Err(ShellError::FileSystem(
                                    FileSystemError::FailedToGetParent,
                                ));
                            }
                        }
                    }
                }
            }
            DirectoryChange::Current => {
                // Nothing to do
            }
            DirectoryChange::Previous => {
                let previous_path =
                    self.previous_working_directory
                        .clone()
                        .ok_or(ShellError::Session(
                            SessionError::NoPreviousWorkingDirectory,
                        ))?;
                let previous_inode = fs.find_absolute_inode(&previous_path.display().to_string());
                if previous_inode.is_some() {
                    self.current_working_directory = previous_path;
                } else {
                    return Err(ShellError::Session(
                        SessionError::PreviousWorkingDirectoryDoesNotExist,
                    ));
                }
            }
        }
        self.previous_working_directory = Some(prev_working_directory);
        Ok(())
    }

    fn get_user_home_directory(&self, fs: &FileSystem) -> PathBuf {
        let user = fs.users.user(self.current_user).expect("User not found");
        PathBuf::from(format!("/home/{}", user.name))
    }

    fn parse_directory_change(
        &self,
        fs: &FileSystem,
        path: &str,
    ) -> Result<DirectoryChange, ShellError> {
        if path == ".." || path == "../" {
            return Ok(DirectoryChange::Parent);
        }
        if path == "." || path == "./" {
            return Ok(DirectoryChange::Current);
        }
        if path.starts_with("/") {
            return Ok(DirectoryChange::Path(path.to_string()));
        }
        if path == "-" {
            return Ok(DirectoryChange::Previous);
        }
        if path == "~" || path.is_empty() {
            return Ok(DirectoryChange::Path(
                self.get_user_home_directory(fs).display().to_string(),
            ));
        }
        if path.starts_with("~/") {
            return Ok(DirectoryChange::Path(format!(
                "{}/{}",
                self.get_user_home_directory(fs).display(),
                // Remove the leading ~ and /
                path.strip_prefix("~/")
                    .expect("Failed to strip prefix that should be there")
            )));
        }
        // If not a special case, it is a relative path
        Ok(DirectoryChange::Path(format!(
            "{}/{}",
            self.current_working_directory.display(),
            path
        )))
    }

    pub fn change_user(&mut self, fs: &FileSystem, user_id: UserId) -> Result<(), ShellError> {
        if fs.users.user(user_id).is_none() {
            return Err(ShellError::Session(SessionError::UserNotFound));
        }
        self.current_user = user_id;
        Ok(())
    }

    pub fn add_to_history(&mut self, command: &str) {
        self.executed_commands.push(command.to_string());
    }

    pub fn get_history(&self) -> Vec<String> {
        self.executed_commands.clone()
    }

    pub fn get_current_working_directory(&self) -> PathBuf {
        self.current_working_directory.clone()
    }

    pub fn get_previous_working_directory(&self) -> Option<PathBuf> {
        self.previous_working_directory.clone()
    }
}

pub enum DirectoryChange {
    Path(String),
    Parent,
    Current,
    Previous,
}
