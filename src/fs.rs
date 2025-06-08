//! Simple representation of a file system.

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use inode::{Inode, content::Directory};
use users::{GroupStore, UserStore};

use crate::{FilePermissions, InodeContent, InodeMetadata, UserId, fs::inode::content::File};

pub mod inode;
pub mod permissions;
pub mod users;

/// The file system
pub struct FileSystem {
    pub root: Rc<RefCell<Inode>>,
    pub users: UserStore,
    pub groups: GroupStore,
    pub current_dir: Weak<RefCell<Inode>>,
    pub current_user: UserId,
}

impl FileSystem {
    pub fn new_with_user(username: &str) -> Self {
        let mut groups = GroupStore::new();
        let mut users = UserStore::new();

        // Create the root user and group
        let root_group_id = groups.add_group("root".to_string());
        let root_user_id = users.add_user("root".to_string());
        let user = users.user_mut(root_user_id).expect("User not found");
        user.add_group(root_group_id);

        // Create the root directory
        let root = Inode::new(
            String::new(),
            InodeContent::Directory(Directory::new()),
            InodeMetadata::new(
                FilePermissions::from_mode(0o755),
                root_user_id,
                root_group_id,
            ),
            None,
        )
        .expect("Failed to create root inode");

        let root = Rc::new(RefCell::new(root));
        root.borrow_mut()
            .add_child(
                "test",
                InodeContent::File(File::new()),
                Rc::downgrade(&root),
            )
            .expect("Failed to add child to root");

        // Create the current user and its home directory
        let current_user_id = users.add_user(username.to_string());
        let current_user = users.user_mut(current_user_id).expect("User not found");
        current_user.add_group(root_group_id);

        Self {
            current_dir: Rc::downgrade(&root),
            root,
            users,
            groups,
            current_user: current_user_id,
        }
    }
}
