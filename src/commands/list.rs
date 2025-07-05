//! Listing and implementation of the supported shell commands.

use strum_macros::EnumIter;

use crate::commands::list;

pub mod cd;
pub mod echo;
pub mod exit;
pub mod history;
pub mod mkdir;
pub mod pwd;
pub mod rm;
pub mod touch;

#[derive(EnumIter, Clone, Copy)]
#[enum_dispatch::enum_dispatch]
pub enum Commands {
    Pwd(list::pwd::PwdCommand),
    Cd(list::cd::ChangeDirectoryCommand),
    History(list::history::HistoryCommand),
    Echo(list::echo::EchoCommand),
    Exit(list::exit::ExitCommand),
    MakeDirectory(list::mkdir::MakeDirectoryCommand),
    Remove(list::rm::RemoveCommand),
    Touch(list::touch::TouchCommand),
}
