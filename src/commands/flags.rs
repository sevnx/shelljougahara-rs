//! Flags definition for a command.

use std::collections::HashSet;

pub struct Flags {
    pub flags: HashSet<FlagSpecification>,
}

pub struct FlagSpecification {
    pub name: &'static str,
    pub short_hand: Option<char>,
    pub required: bool,
    pub arg_type: FlagType,
}

pub enum FlagType {
    Boolean,
    String,
}
