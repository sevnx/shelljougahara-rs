//! Users and groups

use std::collections::HashMap;

pub type UserId = u32;
pub type GroupId = u32;

pub struct UserStore {
    pub users: HashMap<UserId, User>,
    pub next_user_id: UserId,
}

impl UserStore {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
            next_user_id: 0,
        }
    }

    pub fn add_user(&mut self, name: String) -> UserId {
        let id = self.next_user_id;
        self.next_user_id += 1;
        self.users.insert(
            id,
            User {
                id,
                name,
                groups: vec![],
            },
        );
        id
    }

    pub fn user_mut(&mut self, id: UserId) -> Option<&mut User> {
        self.users.get_mut(&id)
    }

    pub fn delete_user(&mut self, id: UserId) {
        self.users.remove(&id);
    }
}

impl Default for UserStore {
    fn default() -> Self {
        Self::new()
    }
}
pub struct User {
    pub id: UserId,
    pub name: String,
    pub groups: Vec<GroupId>,
}

impl User {
    pub fn new(id: UserId, name: String) -> Self {
        Self {
            id,
            name,
            groups: vec![],
        }
    }

    pub fn add_group(&mut self, group_id: GroupId) {
        self.groups.push(group_id);
    }

    pub fn remove_group(&mut self, group_id: GroupId) {
        self.groups.retain(|g| *g != group_id);
    }
}

pub struct GroupStore {
    pub groups: HashMap<GroupId, Group>,
    pub next_group_id: GroupId,
}

impl GroupStore {
    pub fn new() -> Self {
        Self {
            groups: HashMap::new(),
            next_group_id: 0,
        }
    }

    pub fn add_group(&mut self, name: String) -> GroupId {
        let id = self.next_group_id;
        self.next_group_id += 1;
        self.groups.insert(id, Group { id, name });
        id
    }

    pub fn group(&self, id: GroupId) -> Option<&Group> {
        self.groups.get(&id)
    }

    pub fn delete_group(&mut self, id: GroupId) {
        self.groups.remove(&id);
    }
}

impl Default for GroupStore {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Group {
    pub id: GroupId,
    pub name: String,
}
