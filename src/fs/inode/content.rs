//! The content that holds the data of an inode.

use std::rc::Weak;

use super::Inode;

pub enum InodeContent {
    File(File),
    Directory(Directory),
    Link(Weak<Inode>),
}

pub struct File {
    pub content: String,
}

pub struct Directory {
    pub children: Vec<Inode>,
}
