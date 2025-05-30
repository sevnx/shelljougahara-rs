//! The content of a link inode.

use std::rc::Weak;

use crate::Inode;

pub struct Link {
    pub target: Weak<Inode>,
}
