#![allow(dead_code)] // TODO: Remove this once the flags are used
//! Flags definition for a command.

use std::collections::{HashMap, HashSet};

use crate::commands::args::{Argument, ArgumentKind};

pub struct Flags {
    flags: HashMap<String, Argument>,
}

impl Flags {
    pub fn new() -> Self {
        Self {
            flags: HashMap::new(),
        }
    }

    pub fn insert(&mut self, name: &str, arg: Argument) {
        self.flags.insert(name.to_string(), arg);
    }

    pub fn flag(&self, name: &str) -> Option<&Argument> {
        self.flags.get(name)
    }

    pub fn flags(&self) -> &HashMap<String, Argument> {
        &self.flags
    }
}

pub struct FlagDefinition {
    flags: HashSet<FlagSpecification>,
}

impl FlagDefinition {
    pub fn new(flags: HashSet<FlagSpecification>) -> Self {
        Self { flags }
    }

    pub fn get_flag_longhand(&self, name: &str) -> Option<&FlagSpecification> {
        self.flags.iter().find(|f| f.name == name)
    }

    pub fn get_flag_shorthand(&self, name: char) -> Option<&FlagSpecification> {
        self.flags.iter().find(|f| f.short_hand == Some(name))
    }

    pub fn into_flags(self) -> HashSet<FlagSpecification> {
        self.flags
    }
}

pub struct FlagDefinitionBuilder {
    flags: HashSet<FlagSpecification>,
}

impl FlagDefinitionBuilder {
    pub fn new() -> Self {
        Self {
            flags: HashSet::new(),
        }
    }

    pub fn with_flag(mut self, flag: FlagSpecification) -> Self {
        self.flags.insert(flag);
        self
    }

    pub fn into_flag_definition(self) -> FlagDefinition {
        FlagDefinition::new(self.flags)
    }
}

#[derive(Eq, Hash, PartialEq)]
pub struct FlagSpecification {
    pub name: &'static str,
    pub short_hand: Option<char>,
    pub required: bool,
    pub arg_type: ArgumentKind,
}

impl FlagSpecification {
    pub fn new(
        name: &'static str,
        short_hand: Option<char>,
        required: bool,
        arg_type: ArgumentKind,
    ) -> Self {
        Self {
            name,
            short_hand,
            required,
            arg_type,
        }
    }
}
