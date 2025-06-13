//! Inode is a file system object that represents an entry in the file system.

use std::{
    path::PathBuf,
    sync::{Arc, Mutex, Weak},
};

use content::InodeContent;
use metadata::InodeMetadata;

use crate::errors::{FileSystemError, ShellError};

pub mod content;
pub mod metadata;

#[derive(Debug, Clone)]
pub struct Inode {
    pub name: String,
    pub content: InodeContent,
    pub metadata: InodeMetadata,
    pub parent: Option<Weak<Mutex<Inode>>>,
}

impl Inode {
    /// Creates a new inode.
    ///
    /// # Errors
    ///
    /// If the name is empty and a parent is provided, an error is returned
    pub fn new(
        name: String,
        content: InodeContent,
        metadata: InodeMetadata,
        parent: Option<Weak<Mutex<Inode>>>,
    ) -> Result<Self, ShellError> {
        if name.is_empty() && parent.is_some() {
            return Err(ShellError::Internal(
                "Cannot create an inode with an empty name and a parent".to_string(),
            ));
        }

        Ok(Self {
            name,
            content,
            metadata,
            parent,
        })
    }

    /// Returns the path of the inode.
    ///
    /// # Errors
    ///
    /// An error would mean that a parent directory does not exist, which is an internal error.
    pub fn path(&self) -> Result<PathBuf, ShellError> {
        let path = if let Some(parent_weak) = self.parent.as_ref() {
            let parent = parent_weak.upgrade().ok_or(ShellError::Internal(
                "Parent directory should exist".to_string(),
            ))?;
            let mut parent_path = parent.lock().expect("Failed to lock parent inode").path()?;
            parent_path.push(&self.name);
            parent_path
        } else {
            PathBuf::from("/")
        };

        Ok(path)
    }

    /// Creates a new inode and adds it to the directory, returning a reference to the inode.
    ///
    /// # Errors
    ///
    /// The following errors can be returned:
    /// - `ShellError::InternalError` if the inode is not a directory
    /// - `ShellError::FileSystemError` if the child already exists
    pub fn add_child(
        &mut self,
        child_name: &str,
        inode_content: InodeContent,
        parent_ref: Weak<Mutex<Inode>>,
    ) -> Result<Arc<Mutex<Inode>>, ShellError> {
        match self.content {
            InodeContent::Directory(ref mut directory) => {
                let inode = Inode::new(
                    child_name.to_string(),
                    inode_content,
                    self.metadata.clone(),
                    Some(parent_ref),
                )?;
                let inode_ref = Arc::new(Mutex::new(inode));
                directory.add_child(inode_ref.clone())?;
                Ok(inode_ref)
            }
            _ => Err(ShellError::Internal(
                "Tried to add a child to an inode that is not a directory".to_string(),
            )),
        }
    }

    pub fn remove_child(&mut self, child_name: &str) -> Result<(), ShellError> {
        match self.content {
            InodeContent::Directory(ref mut directory) => {
                if !directory.contains(child_name) {
                    return Err(ShellError::FileSystem(FileSystemError::EntryNotFound(
                        child_name.to_string(),
                    )));
                }
                directory.remove_child(child_name);
                Ok(())
            }
            _ => Err(ShellError::Internal(
                "Tried to remove a child from an inode that is not a directory".to_string(),
            )),
        }
    }

    pub fn find_child(&self, child_name: &str) -> Option<Arc<Mutex<Inode>>> {
        match self.content {
            InodeContent::Directory(ref directory) => directory.find_child(child_name),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        FilePermissions,
        fs::inode::content::{Directory, File},
    };

    use super::*;

    /// Creates a simple inode with a given name, content, and parent.
    fn get_simple_inode(
        name: &str,
        content: InodeContent,
        parent: Option<Weak<Mutex<Inode>>>,
    ) -> Result<Arc<Mutex<Inode>>, ShellError> {
        Inode::new(
            name.to_string(),
            content,
            InodeMetadata::new(FilePermissions::from_mode(0o644), 0, 0),
            parent,
        )
        .map(|inode| Arc::new(Mutex::new(inode)))
    }

    #[test]
    fn test_error_empty_inode_creation() {
        let base = get_simple_inode("", InodeContent::Directory(Directory::new()), None)
            .expect("Failed to create base inode");
        let empty_child = get_simple_inode(
            "",
            InodeContent::Directory(Directory::new()),
            Some(Arc::downgrade(&base)),
        );
        assert!(empty_child.is_err());
    }

    #[test]
    fn test_path_construction() {
        let root = get_simple_inode("", InodeContent::Directory(Directory::new()), None)
            .expect("Failed to create root inode");
        let dir = get_simple_inode(
            "test",
            InodeContent::File(File::new()),
            Some(Arc::downgrade(&root)),
        )
        .expect("Failed to create dir inode");
        let file = get_simple_inode(
            "test.txt",
            InodeContent::File(File::new()),
            Some(Arc::downgrade(&dir)),
        )
        .expect("Failed to create file inode");

        assert_eq!(
            root.lock()
                .expect("Failed to lock root inode")
                .path()
                .unwrap(),
            PathBuf::from("/")
        );
        assert_eq!(
            dir.lock()
                .expect("Failed to lock dir inode")
                .path()
                .unwrap(),
            PathBuf::from("/test")
        );
        assert_eq!(
            file.lock()
                .expect("Failed to lock file inode")
                .path()
                .unwrap(),
            PathBuf::from("/test/test.txt")
        );
    }

    #[test]
    fn test_add_and_retrieve_child() {
        let root = get_simple_inode("", InodeContent::Directory(Directory::new()), None)
            .expect("Failed to create root inode");
        root.lock()
            .expect("Failed to lock root inode")
            .add_child(
                "test",
                InodeContent::File(File::new()),
                Arc::downgrade(&root.clone()),
            )
            .expect("Failed to add child");
        assert!(
            root.lock()
                .expect("Failed to lock root inode")
                .find_child("test")
                .is_some()
        );
    }
}
