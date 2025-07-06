//! Representation of a shell session.
//!
//! A session represents a shell instance, a single file system can have multiple sessions.
//! It contains a reference to the file system, and information like the current user and directory.

use std::{
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

use crate::{
    FileSystem, Inode, ShellError, UserId,
    errors::{FileSystemError, SessionError},
    fs::resolver::resolve_path,
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

    pub fn create_file(
        &mut self,
        fs: &mut FileSystem,
        path: &Path,
    ) -> Result<Arc<Mutex<Inode>>, ShellError> {
        let resolved_path = resolve_path(
            path,
            &self.get_user_home_directory(fs),
            &self.current_working_directory,
        );
        fs.create_file(&resolved_path.display().to_string())
    }

    pub fn create_directory(
        &mut self,
        fs: &mut FileSystem,
        path: &Path,
    ) -> Result<Arc<Mutex<Inode>>, ShellError> {
        let resolved_path = resolve_path(
            path,
            &self.get_user_home_directory(fs),
            &self.current_working_directory,
        );
        fs.create_directory(&resolved_path.display().to_string())
    }

    pub fn remove_file(&mut self, fs: &mut FileSystem, path: &Path) -> Result<(), ShellError> {
        let resolved_path = resolve_path(
            path,
            &self.get_user_home_directory(fs),
            &self.current_working_directory,
        );
        fs.remove_inode(&resolved_path.display().to_string())?;
        Ok(())
    }

    pub fn find_inode(&self, fs: &FileSystem, path: &Path) -> Option<Arc<Mutex<Inode>>> {
        let resolved_path = resolve_path(
            path,
            &self.get_user_home_directory(fs),
            &self.current_working_directory,
        );
        fs.find_absolute_inode(&resolved_path.display().to_string())
    }

    pub fn change_directory(&mut self, fs: &FileSystem, path: &Path) -> Result<(), ShellError> {
        let prev_working_directory = self.current_working_directory.clone();

        if path == Path::new("-") {
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
        } else if path != prev_working_directory {
            let resolved_path = resolve_path(
                path,
                &self.get_user_home_directory(fs),
                &prev_working_directory,
            );
            let inode = fs.find_absolute_inode(&resolved_path.display().to_string());
            if inode.is_some() {
                self.current_working_directory = resolved_path;
            } else {
                return Err(ShellError::FileSystem(FileSystemError::DirectoryNotFound(
                    resolved_path.display().to_string(),
                )));
            }
        }
        self.previous_working_directory = Some(prev_working_directory);
        Ok(())
    }

    fn get_user_home_directory(&self, fs: &FileSystem) -> PathBuf {
        let user = fs.get_user(self.current_user).expect("User not found");
        PathBuf::from(format!("/home/{}", user.name))
    }

    pub fn change_user(&mut self, fs: &FileSystem, user_id: UserId) -> Result<(), ShellError> {
        if fs.get_user(user_id).is_none() {
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
