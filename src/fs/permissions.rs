//! Permissions are a set of flags that control the access to a file system object.

pub struct FilePermissions {
    pub mode: u32,
}

impl Default for FilePermissions {
    fn default() -> Self {
        Self { mode: 0o600 }
    }
}

impl FilePermissions {
    pub fn from_mode(mode: u32) -> Self {
        Self { mode }
    }

    pub fn from_permissions(user: Permission, group: Permission, other: Permission) -> Self {
        FilePermissionBuilder::new()
            .user_permission(user.read, user.write, user.execute)
            .group_permission(group.read, group.write, group.execute)
            .other_permission(other.read, other.write, other.execute)
            .build()
    }

    pub fn set_mode(&mut self, mode: u32) {
        self.mode = mode;
    }

    pub fn mode(&self) -> u32 {
        self.mode
    }
}

pub struct Permission {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
}

impl Permission {
    pub fn new(read: bool, write: bool, execute: bool) -> Self {
        Self {
            read,
            write,
            execute,
        }
    }
}

pub struct FilePermissionBuilder {
    pub user: Permission,
    pub group: Permission,
    pub other: Permission,
}

impl FilePermissionBuilder {
    pub fn new() -> Self {
        Self {
            user: Permission::new(false, false, false),
            group: Permission::new(false, false, false),
            other: Permission::new(false, false, false),
        }
    }

    pub fn user_permission(mut self, read: bool, write: bool, execute: bool) -> Self {
        self.user = Permission::new(read, write, execute);
        self
    }

    pub fn group_permission(mut self, read: bool, write: bool, execute: bool) -> Self {
        self.group = Permission::new(read, write, execute);
        self
    }

    pub fn other_permission(mut self, read: bool, write: bool, execute: bool) -> Self {
        self.other = Permission::new(read, write, execute);
        self
    }

    pub fn build(&self) -> FilePermissions {
        let mut mode = 0;
        if self.user.read {
            mode |= 0o400;
        }
        if self.user.write {
            mode |= 0o200;
        }
        if self.user.execute {
            mode |= 0o100;
        }
        if self.group.read {
            mode |= 0o040;
        }
        if self.group.write {
            mode |= 0o020;
        }
        if self.group.execute {
            mode |= 0o010;
        }
        if self.other.read {
            mode |= 0o004;
        }
        if self.other.write {
            mode |= 0o002;
        }
        if self.other.execute {
            mode |= 0o001;
        }
        FilePermissions::from_mode(mode)
    }
}
