//! The metadata of an inode.

use chrono::{DateTime, Utc};

use crate::fs::{
    permissions::FilePermissions,
    users::{GroupId, UserId},
};

#[derive(Debug, Clone)]
pub struct InodeMetadata {
    pub permissions: FilePermissions,
    pub owner: UserId,
    pub group: GroupId,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl InodeMetadata {
    #[must_use] pub fn new(permissions: FilePermissions, owner: UserId, group: GroupId) -> Self {
        Self {
            permissions,
            owner,
            group,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}
