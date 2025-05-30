//! The list of shell commands supported by the shell.

use crate::commands::Command;

mod pwd;

#[enum_dispatch::enum_dispatch]
pub enum CommandList {}
