//! The content of a directory inode.

use crate::Inode;

pub struct Directory {
    pub children: Vec<Inode>,
}

impl Directory {
    pub fn new() -> Self {
        Self { children: vec![] }
    }

    pub fn add_child(&mut self, child: Inode) {
        self.children.push(child);
    }

    pub fn remove_child(&mut self, child: Inode) {
        self.children.retain(|c| c.name != child.name);
    }
}

impl Default for Directory {
    fn default() -> Self {
        Self::new()
    }
}
