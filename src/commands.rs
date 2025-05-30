//! Commands, the things that the shell can execute.

use crate::{FileSystem, errors::CommandError};
use flags::Flags;

// Re-export
pub use list::CommandList;

mod flags;
mod list;

#[enum_dispatch::enum_dispatch(CommandList)]
pub trait Command {
    fn name(&self) -> &str;
    fn flags(&self) -> Flags;
    /// Executes a command
    ///
    /// # Returns
    ///
    /// - `Ok(CommandOutput)` on successful execution, where the output contains
    ///   the command's result or any user-facing error messages.
    /// - `Err` when an unexpected internal error occurs that prevents command execution.
    fn execute(&self, args: &[String], fs: &mut FileSystem) -> Result<CommandOutput, CommandError>;
}

pub struct CommandOutput(pub String);
