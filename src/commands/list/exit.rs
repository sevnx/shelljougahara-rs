//! The exit command.

use crate::{
    commands::{Command, CommandOutput, flags::Flags},
    errors::ShellError,
};

#[derive(Default)]
pub struct ExitCommand;

impl Command for ExitCommand {
    fn name(&self) -> &'static str {
        "exit"
    }

    fn flags(&self) -> Flags {
        Flags::new()
    }

    fn execute(
        &self,
        _: &[String],
        shell: &mut crate::shell::Shell,
    ) -> Result<CommandOutput, ShellError> {
        shell.active = false;
        Ok(CommandOutput(None))
    }
}
