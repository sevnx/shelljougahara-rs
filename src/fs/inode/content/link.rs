//! The content of a link inode.

use std::sync::{Mutex, Weak};

use crate::Inode;

pub struct Link {
    pub target: Weak<Mutex<Inode>>,
}
