//! Simple representation of a file system.

use std::{
    collections::HashMap,
    rc::{Rc, Weak},
};

use inode::Inode;
use users::{Group, GroupId, User, UserId};

pub mod inode;
pub mod permissions;
pub mod users;

/// The file system
pub struct FileSystem {
    pub root: Rc<Inode>,
    pub users: HashMap<UserId, User>,
    pub groups: HashMap<GroupId, Group>,
    pub current_dir: Weak<Inode>,
}

impl FileSystem {}
