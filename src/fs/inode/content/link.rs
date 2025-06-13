//! The content of a link inode.

use std::sync::{Mutex, Weak};

use crate::Inode;

#[derive(Debug, Clone)]
pub struct Link {
    pub target: Weak<Mutex<Inode>>,
}
