//! The content of a link inode.

use std::sync::{Mutex, Weak};

use crate::{Inode, fs::inode::size::Size};

#[derive(Debug, Clone)]
pub struct Link {
    pub target_path: String,
    pub target_inode: Weak<Mutex<Inode>>,
}

impl Size for Link {
    fn size(&self) -> u64 {
        self.target_path.len() as u64
    }
}
