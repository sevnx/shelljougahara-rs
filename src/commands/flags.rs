//! Flags definition for a command.

use std::collections::HashSet;

pub struct Flags {
    pub flags: HashSet<FlagSpecification>,
}

impl Flags {
    pub fn new() -> Self {
        Self {
            flags: HashSet::new(),
        }
    }
}

pub struct FlagSpecification {
    pub name: &'static str,
    pub short_hand: Option<char>,
    pub required: bool,
    pub arg_type: FlagType,
}

impl FlagSpecification {
    pub fn new(
        name: &'static str,
        short_hand: Option<char>,
        required: bool,
        arg_type: FlagType,
    ) -> Self {
        Self {
            name,
            short_hand,
            required,
            arg_type,
        }
    }
}

pub enum FlagType {
    Boolean,
    String,
}
