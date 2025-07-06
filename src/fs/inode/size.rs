//! Size utilities for inodes.

pub trait Size {
    /// Returns the size of the inode in bytes.
    fn size(&self) -> u64;
}
