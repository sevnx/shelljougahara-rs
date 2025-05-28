//! Users and groups

pub type UserId = u32;
pub type GroupId = u32;

pub struct User {
    pub id: UserId,
    pub name: String,
    pub groups: Vec<GroupId>,
}

pub struct Group {
    pub id: GroupId,
    pub name: String,
}
