//! The virtual file system used by the shell.

use std::sync::{Arc, Mutex};

use inode::{Inode, content::Directory};
use users::{GroupStore, UserStore};

use crate::{
    FilePermissions, InodeContent, InodeMetadata, UserId,
    errors::{FileSystemError, ShellError},
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
}

impl FileSystem {
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

impl Default for FileSystem {
    fn default() -> Self {
        Self::new()
    }
}
