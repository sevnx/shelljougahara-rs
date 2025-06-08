//! The content of a directory inode.

use std::{
    collections::{HashMap, hash_map::Entry},
    rc::Rc,
};

use crate::{Inode, errors::OperationResult};

pub struct Directory {
    pub children: HashMap<String, Rc<Inode>>,
}

impl Directory {
    pub fn new() -> Self {
        Self {
            children: HashMap::new(),
        }
    }

    pub fn add_child(&mut self, child: Rc<Inode>) -> OperationResult {
        match self.children.entry(child.name.clone()) {
            Entry::Vacant(entry) => {
                entry.insert(child);
                OperationResult::Success
            }
            Entry::Occupied(_) => OperationResult::Failure,
        }
    }

    pub fn exists(&self, name: &str) -> bool {
        self.children.contains_key(name)
    }

    pub fn remove_child(&mut self, child_name: &str) {
        self.children.remove(child_name);
    }

    pub fn find_child(&self, name: &str) -> Option<&Inode> {
        self.children.get(name).map(|inode| inode.as_ref())
    }
}

impl Default for Directory {
    fn default() -> Self {
        Self::new()
    }
}
