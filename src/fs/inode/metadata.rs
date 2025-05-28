//! The metadata of an inode.

use chrono::{DateTime, Utc};

use crate::fs::{
    permissions::FilePermissions,
    users::{GroupId, UserId},
};

pub struct InodeMetadata {
    pub permissions: FilePermissions,
    pub owner: UserId,
    pub group: GroupId,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
