//! Simple representation of a file system.

use std::rc::{Rc, Weak};

use inode::{Inode, content::Directory};
use users::{GroupStore, UserStore};

use crate::{FilePermissions, InodeContent, InodeMetadata};

pub mod inode;
pub mod permissions;
pub mod users;

/// The file system
pub struct FileSystem {
    pub root: Rc<Inode>,
    pub users: UserStore,
    pub groups: GroupStore,
    pub current_dir: Weak<Inode>,
}

impl FileSystem {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for FileSystem {
    fn default() -> Self {
        let mut groups = GroupStore::new();
        let root_group_id = groups.add_group("root".to_string());

        let mut users = UserStore::new();
        let root_user_id = users.add_user("root".to_string());
        let user = users.user_mut(root_user_id).expect("User not found");
        user.add_group(root_group_id);

        let root = Inode::new(
            "/".to_string(),
            InodeContent::Directory(Directory::new()),
            InodeMetadata::new(
                FilePermissions::from_mode(0o755),
                root_user_id,
                root_group_id,
            ),
            None,
        );

        let root_dir = Rc::new(root);

        Self {
            current_dir: Rc::downgrade(&root_dir),
            root: root_dir,
            users,
            groups,
        }
    }
}
