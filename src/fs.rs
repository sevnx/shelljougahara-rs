//! The virtual file system used by the shell.

use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use inode::{Inode, content::Directory};
use users::{GroupStore, UserStore};

use crate::{
    FilePermissions, InodeContent, InodeMetadata, UserId,
    errors::{FileSystemError, ShellError},
};

pub mod inode;
pub mod permissions;
pub mod resolver;
pub mod users;

/// The file system
#[derive(Debug, Clone)]
pub struct FileSystem {
    pub root: Arc<Mutex<Inode>>,
    pub users: UserStore,
    pub groups: GroupStore,
}

impl FileSystem {
    #[must_use]
    pub fn new() -> Self {
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
            root,
            users,
            groups,
        }
    }

    pub fn create_directory(&mut self, path: &str) -> Result<Arc<Mutex<Inode>>, ShellError> {
        let (components, last_component) = path.split_at(path.rfind('/').unwrap_or(0) + 1);
        let mut current_inode = Some(self.root.clone());
        for component in components.split('/') {
            let inode = current_inode.take().expect("Inode is empty");
            let inode_copy = inode.clone();
            let inner_inode = inode.lock().expect("Failed to lock inner inode");
            match component {
                "" | "." => {
                    current_inode = Some(inode_copy);
                }
                ".." => match inner_inode.parent() {
                    Some(parent) => {
                        let parent_arc = parent.upgrade().expect("Failed to upgrade parent");
                        current_inode = Some(parent_arc);
                    }
                    None => {
                        if inner_inode.path()? == PathBuf::from("/") {
                            current_inode = Some(inode_copy);
                        } else {
                            return Err(ShellError::FileSystem(
                                FileSystemError::DirectoryNotFound(
                                    "Parent directory does not exist".to_string(),
                                ),
                            ));
                        }
                    }
                },
                path => match inner_inode.find_child(path) {
                    Some(child_inode) => {
                        let child_content = child_inode
                            .lock()
                            .expect("Failed to lock child inode")
                            .content
                            .clone();
                        match child_content {
                            InodeContent::Directory(_) => {
                                current_inode = Some(child_inode);
                            }
                            _ => {
                                return Err(ShellError::FileSystem(
                                    FileSystemError::EntryAlreadyExists(path.to_string()),
                                ));
                            }
                        }
                    }
                    None => {
                        return Err(ShellError::FileSystem(FileSystemError::DirectoryNotFound(
                            path.to_string(),
                        )));
                    }
                },
            }
        }
        let current_inode = current_inode.expect("Failed to lock current inode");
        current_inode
            .lock()
            .expect("Failed to lock inode")
            .add_child(
                last_component,
                InodeContent::Directory(Directory::new()),
                Arc::downgrade(&current_inode),
            )
    }

    pub fn add_user(&mut self, username: &str) -> Result<UserId, ShellError> {
        if self.find_absolute_inode("/home").is_none() {
            self.create_directory("/home")?;
        }
        let user_id = self.users.add_user(username.to_string());
        let user_group_id = self.groups.add_group(username.to_string());
        let user = self.users.user_mut(user_id).expect("User not found");
        user.add_group(user_group_id);
        self.create_directory(&format!("/home/{}", username))?;
        Ok(user_id)
    }

    #[must_use]
    pub fn find_absolute_inode(&self, path: &str) -> Option<Arc<Mutex<Inode>>> {
        find_relative_inode(self.root.clone(), path)
    }

    pub fn remove_inode(&mut self, path: &str) -> Result<(), ShellError> {
        let inode = self
            .find_absolute_inode(path)
            .ok_or(ShellError::FileSystem(FileSystemError::EntryNotFound(
                path.to_string(),
            )))?;
        inode
            .lock()
            .expect("Failed to lock inode")
            .remove_child(path)?;
        Ok(())
    }
}

impl Default for FileSystem {
    fn default() -> Self {
        Self::new()
    }
}

pub fn find_relative_inode(
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
