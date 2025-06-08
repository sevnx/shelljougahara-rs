//! The echo command, printing the arguments.

use crate::{
    commands::{Command, CommandOutput, flags::Flags},
    errors::ShellError,
};

#[derive(Default)]
pub struct EchoCommand;

impl Command for EchoCommand {
    fn name(&self) -> &'static str {
        "echo"
    }

    fn flags(&self) -> Flags {
        Flags::new()
    }

    fn execute(
        &self,
        args: &[String],
        _: &mut crate::shell::Shell,
    ) -> Result<CommandOutput, ShellError> {
        Ok(CommandOutput(args.join(" ")))
    }
}
