//! The virtual file system used by the shell.

use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use inode::{Inode, content::Directory};
use users::{GroupStore, UserStore};

use crate::{
    FilePermissions, Group, GroupId, InodeContent, InodeMetadata, User, UserId,
    errors::{FileSystemError, ShellError},
    fs::inode::content::File,
};

pub mod inode;
pub mod permissions;
pub mod resolver;
pub mod users;

/// The file system
#[derive(Debug, Clone)]
pub struct FileSystem {
    pub root: Arc<Mutex<Inode>>,
    users: UserStore,
    groups: GroupStore,
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

    pub fn get_user(&self, user_id: UserId) -> Option<&User> {
        self.users.user(user_id)
    }

    pub fn get_group(&self, group_id: GroupId) -> Option<&Group> {
        self.groups.group(group_id)
    }

    pub fn create_file(&mut self, path: &str) -> Result<Arc<Mutex<Inode>>, ShellError> {
        let (components, last_component) = path.split_at(path.rfind('/').unwrap_or(0) + 1);
        let current_inode = self.path_from_components(components)?;
        current_inode
            .lock()
            .expect("Failed to lock inode")
            .add_child(
                last_component,
                InodeContent::File(File::new()),
                Arc::downgrade(&current_inode),
            )
    }

    pub fn create_directory(&mut self, path: &str) -> Result<Arc<Mutex<Inode>>, ShellError> {
        let (components, last_component) = path.split_at(path.rfind('/').unwrap_or(0) + 1);
        let current_inode = self.path_from_components(components)?;
        current_inode
            .lock()
            .expect("Failed to lock inode")
            .add_child(
                last_component,
                InodeContent::Directory(Directory::new()),
                Arc::downgrade(&current_inode),
            )
    }

    fn path_from_components(&self, path: &str) -> Result<Arc<Mutex<Inode>>, ShellError> {
        let mut current_inode = Some(self.root.clone());
        // We separate the path into components (which should be directories) and the last component (which is the path)
        let (components, _) = path.split_at(path.rfind('/').unwrap_or(0) + 1);
        let mut components_iter = components.split('/').peekable();
        while let Some(component) = components_iter.next() {
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
                        let path = inner_inode.path()?;
                        if path == PathBuf::from("/") {
                            current_inode = Some(inode_copy);
                        } else {
                            return Err(ShellError::FileSystem(
                                FileSystemError::DirectoryNotFound(format!(
                                    "Failed to get parent of {}",
                                    path.display()
                                )),
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
                                if components_iter.peek().is_some() {
                                    return Err(ShellError::FileSystem(
                                        FileSystemError::NotADirectory(path.to_string()),
                                    ));
                                } else {
                                    return Err(ShellError::FileSystem(
                                        FileSystemError::EntryAlreadyExists(path.to_string()),
                                    ));
                                }
                            }
                        }
                    }
                    None => {
                        return Err(ShellError::FileSystem(FileSystemError::EntryNotFound(
                            path.to_string(),
                        )));
                    }
                },
            }
        }

        Ok(current_inode.expect("Inode is empty"))
    }

    pub fn add_user(&mut self, username: &str) -> Result<UserId, ShellError> {
        if self.find_absolute_inode("/home").is_none() {
            self.create_directory("/home")?;
        }

        if self.users.find_by_username(username).is_some() {
            return Err(ShellError::FileSystem(FileSystemError::EntryAlreadyExists(
                username.to_string(),
            )));
        }

        let user_id = self.users.add_user(username.to_string());
        let user_group_id = self.groups.add_group(username.to_string());
        let user = self.users.user_mut(user_id).expect("User not found");
        user.add_group(user_group_id);
        let user_home_dir = self.create_directory(&format!("/home/{username}"))?;
        let mut user_home_dir_inode = user_home_dir
            .lock()
            .expect("Failed to lock user home directory");
        user_home_dir_inode.metadata.group = user_group_id;
        user_home_dir_inode.metadata.owner = user_id;
        Ok(user_id)
    }

    #[must_use]
    pub fn find_absolute_inode(&self, path: &str) -> Option<Arc<Mutex<Inode>>> {
        find_relative_inode(self.root.clone(), path)
    }

    pub fn remove_inode(&mut self, path: &str) -> Result<(), ShellError> {
        let (parent, child) = path.split_at(path.rfind('/').unwrap_or(0) + 1);

        if let Some(parent_inode) = self.find_absolute_inode(parent) {
            let mut parent_inode = parent_inode.lock().expect("Failed to lock parent inode");
            if let InodeContent::Directory(parent_dir) = &mut parent_inode.content {
                println!("Removing child: {child:?} from {parent:?}");
                parent_dir.remove_child(child);
            }
        } else {
            return Err(ShellError::FileSystem(FileSystemError::DirectoryNotFound(
                "Parent directory does not exist".to_string(),
            )));
        }

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
