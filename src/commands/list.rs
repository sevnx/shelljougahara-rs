//! The list of shell commands supported by the shell.

use strum_macros::EnumIter;

use crate::commands::list;
pub mod pwd;

#[derive(EnumIter)]
#[enum_dispatch::enum_dispatch]
pub enum CommandList {
    Pwd(list::pwd::PwdCommand),
}
