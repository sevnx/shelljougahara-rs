//! The list of shell commands supported by the shell.

use strum_macros::EnumIter;

use crate::commands::list;

pub mod cd;
pub mod echo;
pub mod exit;
pub mod history;
pub mod pwd;

#[derive(EnumIter)]
#[enum_dispatch::enum_dispatch]
pub enum CommandList {
    Pwd(list::pwd::PwdCommand),
    Cd(list::cd::ChangeDirectoryCommand),
    History(list::history::HistoryCommand),
    Echo(list::echo::EchoCommand),
    Exit(list::exit::ExitCommand),
}
