//! The content of a directory inode.

use std::{
    collections::{HashMap, hash_map::Entry},
    sync::{Arc, Mutex},
};

use crate::{
    Inode,
    errors::{FileSystemError, ShellError},
    fs::inode::size::Size,
};

#[derive(Debug, Clone)]
pub struct Directory {
    pub children: HashMap<String, Arc<Mutex<Inode>>>,
}

impl Directory {
    pub fn new() -> Self {
        Self {
            children: HashMap::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.children.is_empty()
    }

    pub fn add_child(&mut self, child: &Arc<Mutex<Inode>>) -> Result<(), ShellError> {
        let name = child.lock().expect("Failed to lock inode").name.clone();
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

    pub fn find_child(&self, name: &str) -> Option<Arc<Mutex<Inode>>> {
        self.children.get(name).cloned()
    }
}

impl Size for Directory {
    fn size(&self) -> u64 {
        // Mimick Linux directory entry size, even if it isn't the actual size
        const DIR_ENTRY_SIZE: u64 = 4096;
        DIR_ENTRY_SIZE
    }
}

impl Default for Directory {
    fn default() -> Self {
        Self::new()
    }
}
