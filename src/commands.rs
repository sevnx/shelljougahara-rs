//! Commands, the things that the shell can execute.

use std::{collections::HashMap, sync::OnceLock};

use crate::{commands::list::CommandList, errors::ShellError};
use flags::Flags;
use strum::IntoEnumIterator;

mod flags;
mod list;

static COMMANDS: OnceLock<HashMap<&str, CommandList>> = OnceLock::new();

pub fn get_commands() -> &'static HashMap<&'static str, CommandList> {
    COMMANDS.get_or_init(|| CommandList::iter().map(|cmd| (cmd.name(), cmd)).collect())
}

#[enum_dispatch::enum_dispatch(CommandList)]
pub trait Command {
    fn name(&self) -> &'static str;
    fn flags(&self) -> Flags;
    /// Executes a command
    ///
    /// # Returns
    ///
    /// - `Ok(CommandOutput)` on successful execution, where the output contains
    ///   the command's result or any user-facing error messages.
    /// - `Err` when an unexpected internal error occurs that prevents command execution.
    fn execute(
        &self,
        args: &[String],
        shell: &mut crate::shell::Shell,
    ) -> Result<CommandOutput, ShellError>;
}

pub struct CommandOutput(pub String);
