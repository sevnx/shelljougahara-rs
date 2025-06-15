//! Simple representation of a file system.

use std::sync::{Arc, Mutex, Weak};

use inode::{Inode, content::Directory};
use users::{GroupStore, UserStore};

use crate::{
    FilePermissions, InodeContent, InodeMetadata, UserId,
    errors::{FileSystemError, ShellError, UserError},
};

pub mod inode;
pub mod permissions;
pub mod users;

/// The file system
#[derive(Debug, Clone)]
pub struct FileSystem {
    pub root: Arc<Mutex<Inode>>,
    pub users: UserStore,
    pub groups: GroupStore,
    pub current_dir: Weak<Mutex<Inode>>,
    pub current_user: UserId,
}

impl FileSystem {
    fn new_as_root() -> Self {
        let mut groups = GroupStore::new();
        let mut users = UserStore::new();

        // Create the root user and group
        let root_group_id = groups.add_group("root".to_string());
        let root_user_id = users.add_user("root".to_string());
        let user = users.user_mut(root_user_id).expect("User not found");
        user.add_group(root_group_id);

        // Create the root directory
        let root = Arc::new(Mutex::new(
            Inode::new(
                String::new(),
                InodeContent::Directory(Directory::new()),
                InodeMetadata::new(
                    FilePermissions::from_mode(0o755),
                    root_user_id,
                    root_group_id,
                ),
                None,
            )
            .expect("Failed to create root inode"),
        ));

        Self {
            current_dir: Arc::downgrade(&root),
            root,
            users,
            groups,
            current_user: root_user_id,
        }
    }

    /// Creates a new file system for a given user, creating their home directory.
    pub fn new_with_user(username: &str) -> Self {
        let mut file_system = Self::new_as_root();
        let user_id = file_system.add_user(username).expect("Failed to add user");
        file_system
            .change_user(user_id)
            .expect("Failed to change user");
        file_system
            .change_directory("~")
            .expect("Failed to change directory");
        file_system
    }

    fn create_home_directory(&mut self) -> Result<Arc<Mutex<Inode>>, ShellError> {
        let mut root_inode = self.root.lock().expect("Failed to lock root inode");
        root_inode.add_child(
            "home",
            InodeContent::Directory(Directory::new()),
            Arc::downgrade(&self.root),
        )
    }

    pub fn add_user(&mut self, username: &str) -> Result<UserId, ShellError> {
        let home_directory = self
            .find_absolute_inode("/home")
            .or_else(|| self.create_home_directory().ok())
            .ok_or_else(|| {
                ShellError::FileSystem(FileSystemError::DirectoryNotFound(
                    "Home directory does not exist".to_string(),
                ))
            })?;
        let user_id = self.users.add_user(username.to_string());
        let user_group_id = self.groups.add_group(username.to_string());
        let user = self.users.user_mut(user_id).expect("User not found");
        user.add_group(user_group_id);
        home_directory
            .lock()
            .expect("Failed to lock home directory")
            .add_child(
                username,
                InodeContent::Directory(Directory::new()),
                Arc::downgrade(&home_directory),
            )?;
        Ok(user_id)
    }

    pub fn change_user(&mut self, user_id: UserId) -> Result<(), ShellError> {
        if self.users.user(user_id).is_none() {
            Err(ShellError::User(UserError::UserNotFound))
        } else {
            self.current_user = user_id;
            Ok(())
        }
    }

    pub fn change_directory(&mut self, path: &str) -> Result<(), ShellError> {
        let path_map_to_err = |option: Option<Arc<Mutex<Inode>>>| {
            if let Some(dir) = option {
                Ok(dir)
            } else {
                Err(ShellError::FileSystem(FileSystemError::DirectoryNotFound(
                    "Directory does not exist".to_string(),
                )))
            }
        };

        let directory_change =
            parse_directory_change(path).expect("Failed to parse directory change");
        let new_dir = match directory_change {
            DirectoryChange::Absolute(path) => Some(path_map_to_err(
                self.find_relative_inode(self.root.clone(), &path),
            )?),
            DirectoryChange::Relative(path) => {
                let current_dir = self.current_dir.upgrade().ok_or_else(|| {
                    ShellError::FileSystem(FileSystemError::DirectoryNotFound(
                        "Current directory does not exist".to_string(),
                    ))
                })?;
                Some(path_map_to_err(
                    self.find_relative_inode(current_dir, &path),
                )?)
            }
            DirectoryChange::Home(path) => match path.chars().nth(0) {
                Some('/') | None => {
                    let current_user = self.users.user(self.current_user).expect("User not found");
                    let home_dir = self.find_absolute_inode("/home").ok_or_else(|| {
                        ShellError::FileSystem(FileSystemError::DirectoryNotFound(
                            "Home directory does not exist".to_string(),
                        ))
                    })?;
                    let user_dir =
                        path_map_to_err(self.find_relative_inode(home_dir, &current_user.name))?;
                    if path.is_empty() {
                        Some(user_dir)
                    } else {
                        Some(path_map_to_err(self.find_relative_inode(user_dir, &path))?)
                    }
                }
                Some(_) => return Err(ShellError::FileSystem(FileSystemError::IncorrectPath)),
            },
            DirectoryChange::Parent => {
                let current_dir = self.current_dir.upgrade().ok_or_else(|| {
                    ShellError::FileSystem(FileSystemError::DirectoryNotFound(
                        "Current directory does not exist".to_string(),
                    ))
                })?;
                if let Some(parent_dir) = current_dir
                    .lock()
                    .expect("Failed to lock current directory")
                    .parent
                    .clone()
                {
                    let parent_dir = parent_dir.upgrade().ok_or_else(|| {
                        ShellError::FileSystem(FileSystemError::DirectoryNotFound(
                            "Parent directory does not exist".to_string(),
                        ))
                    })?;
                    Some(parent_dir)
                } else {
                    // We are already at the root directory
                    None
                }
            }
            DirectoryChange::Current => None,
            DirectoryChange::Previous => {
                // TODO: History implementation (if even needed)
                None
            }
        };
        if let Some(new_dir) = new_dir {
            self.current_dir = Arc::downgrade(&new_dir);
        }
        Ok(())
    }

    pub fn find_absolute_inode(&self, path: &str) -> Option<Arc<Mutex<Inode>>> {
        self.find_relative_inode(self.root.clone(), path)
    }

    pub fn find_relative_inode(
        &self,
        base: Arc<Mutex<Inode>>,
        relative_path: &str,
    ) -> Option<Arc<Mutex<Inode>>> {
        let mut current_inode = base;
        for component in relative_path.split('/') {
            if component.is_empty() {
                continue;
            }
            let child_inode = current_inode
                .lock()
                .expect("Failed to lock inode")
                .find_child(component);
            match child_inode {
                Some(inode) => current_inode = inode,
                None => return None,
            }
        }
        Some(current_inode)
    }
}

pub enum DirectoryChange {
    Absolute(String),
    Relative(String),
    Home(String),
    Parent,
    Current,
    Previous,
}

pub fn parse_directory_change(path: &str) -> Result<DirectoryChange, ShellError> {
    // Handle special cases (ugly)
    // TODO: Handle this better
    if path == ".." || path == "../" {
        return Ok(DirectoryChange::Parent);
    }
    if path == "." || path == "./" {
        return Ok(DirectoryChange::Current);
    }
    if path == "~" || path == "~/" || path.is_empty() {
        return Ok(DirectoryChange::Home(String::new()));
    }
    if path == "-" {
        return Ok(DirectoryChange::Previous);
    }

    match path.chars().next() {
        Some('/') => Ok(DirectoryChange::Absolute(path.to_string())),
        Some('~') => Ok(DirectoryChange::Home(path[1..].to_string())),
        Some(_) => Ok(DirectoryChange::Relative(path.to_string())),
        None => Ok(DirectoryChange::Home(String::new())),
    }
}
