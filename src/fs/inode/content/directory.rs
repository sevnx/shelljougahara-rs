//! The content of a directory inode.

use std::{
    cell::RefCell,
    collections::{HashMap, hash_map::Entry},
    rc::Rc,
};

use crate::{
    Inode,
    errors::{FileSystemError, ShellError},
};

pub struct Directory {
    pub children: HashMap<String, Rc<RefCell<Inode>>>,
}

impl Directory {
    pub fn new() -> Self {
        Self {
            children: HashMap::new(),
        }
    }

    pub fn add_child(&mut self, child: Rc<RefCell<Inode>>) -> Result<(), ShellError> {
        let name = child.borrow().name.clone();
        match self.children.entry(name.clone()) {
            Entry::Vacant(entry) => {
                entry.insert(child.clone());
                Ok(())
            }
            Entry::Occupied(_) => Err(ShellError::FileSystem(FileSystemError::EntryAlreadyExists(
                name,
            ))),
        }
    }

    pub fn contains(&self, name: &str) -> bool {
        self.children.contains_key(name)
    }

    pub fn remove_child(&mut self, child_name: &str) {
        self.children.remove(child_name);
    }

    pub fn find_child(&self, name: &str) -> Option<Rc<RefCell<Inode>>> {
        self.children.get(name).cloned()
    }
}

impl Default for Directory {
    fn default() -> Self {
        Self::new()
    }
}
