//! The print working directory command.

use crate::{
    commands::{Command, CommandOutput, flags::Flags},
    errors::ShellError,
};

#[derive(Default)]
pub struct PwdCommand;

impl Command for PwdCommand {
    fn name(&self) -> &'static str {
        "pwd"
    }

    fn flags(&self) -> Flags {
        // TODO: Maybe add `-L` and `-P` flags.
        Flags::new()
    }

    fn execute(
        &self,
        _: &[String],
        shell: &mut crate::shell::Shell,
    ) -> Result<CommandOutput, ShellError> {
        let path = shell
            .current_session
            .get_current_working_directory()
            .display()
            .to_string();
        Ok(CommandOutput(Some(path)))
    }
}
