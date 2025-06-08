//! Inode is a file system object that represents an entry in the file system.

use std::{
    cell::RefCell,
    path::PathBuf,
    rc::{Rc, Weak},
};

use content::InodeContent;
use metadata::InodeMetadata;

use crate::errors::OperationResult;

pub mod content;
pub mod metadata;

pub struct Inode {
    pub name: String,
    pub content: InodeContent,
    pub metadata: InodeMetadata,
    pub parent: Option<Weak<RefCell<Inode>>>,
}

impl Inode {
    pub fn new(
        name: String,
        content: InodeContent,
        metadata: InodeMetadata,
        parent: Option<Weak<RefCell<Inode>>>,
    ) -> Self {
        if name.is_empty() && parent.is_some() {
            panic!("Cannot create an inode with an empty name and a parent");
        }

        Self {
            name,
            content,
            metadata,
            parent,
        }
    }

    pub fn path(&self) -> Result<PathBuf, String> {
        let path = if let Some(parent_weak) = self.parent.as_ref() {
            let parent = parent_weak
                .upgrade()
                .ok_or("Parent directory does not exist")?;
            let mut parent_path = parent.borrow().path()?;
            parent_path.push(&self.name);
            parent_path
        } else {
            PathBuf::from("/")
        };

        Ok(path)
    }

    pub fn add_child(
        &mut self,
        child_name: &str,
        inode_content: InodeContent,
        parent_ref: Weak<RefCell<Inode>>,
    ) -> Result<OperationResult, String> {
        match self.content {
            InodeContent::Directory(ref mut directory) => {
                let inode = Rc::new(Inode::new(
                    child_name.to_string(),
                    inode_content,
                    self.metadata.clone(),
                    Some(parent_ref),
                ));
                Ok(directory.add_child(inode))
            }
            _ => Err(format!("Cannot add a child to a {}", self.name)),
        }
    }

    pub fn remove_child(&mut self, child_name: &str) -> Result<OperationResult, String> {
        match self.content {
            InodeContent::Directory(ref mut directory) => {
                if !directory.exists(child_name) {
                    return Err(format!(
                        "Child with name {} not found in {}",
                        child_name, self.name
                    ));
                }
                directory.remove_child(child_name);
                Ok(OperationResult::Success)
            }
            _ => Err(format!("Cannot remove a child from a {}", self.name)),
        }
    }

    pub fn find_child(&self, child_name: &str) -> Option<&Inode> {
        match self.content {
            InodeContent::Directory(ref directory) => directory.find_child(child_name),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::{
        FilePermissions,
        fs::inode::content::{Directory, File},
    };

    use super::*;

    fn get_simple_inode(
        name: &str,
        content: InodeContent,
        parent: Option<Weak<RefCell<Inode>>>,
    ) -> Rc<RefCell<Inode>> {
        Rc::new(RefCell::new(Inode::new(
            name.to_string(),
            content,
            InodeMetadata::new(FilePermissions::from_mode(0o644), 0, 0),
            parent,
        )))
    }

    #[test]
    fn test_path_construction() {
        let root = get_simple_inode("", InodeContent::Directory(Directory::new()), None);
        let dir = get_simple_inode(
            "test",
            InodeContent::File(File::new()),
            Some(Rc::downgrade(&root)),
        );
        let file = get_simple_inode(
            "test.txt",
            InodeContent::File(File::new()),
            Some(Rc::downgrade(&dir)),
        );

        assert_eq!(root.borrow().path().unwrap(), PathBuf::from("/"));
        assert_eq!(dir.borrow().path().unwrap(), PathBuf::from("/test"));
        assert_eq!(
            file.borrow().path().unwrap(),
            PathBuf::from("/test/test.txt")
        );
    }

    #[test]
    fn test_add_child() {
        let root = get_simple_inode("", InodeContent::Directory(Directory::new()), None);
        root.borrow_mut()
            .add_child(
                "test",
                InodeContent::File(File::new()),
                Rc::downgrade(&root),
            )
            .unwrap();
        assert!(root.borrow().find_child("test").is_some());
    }
}
