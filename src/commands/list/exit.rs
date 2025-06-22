//! The exit command.

use crate::{
    commands::{
        Argument, CommandOutput, ExecutableCommand, Flags, args::ArgumentKind,
        flags::FlagDefinition,
    },
    errors::ShellError,
};

#[derive(Default, Clone, Copy)]
pub struct ExitCommand;

impl ExecutableCommand for ExitCommand {
    fn name(&self) -> &'static str {
        "exit"
    }

    fn flags(&self) -> FlagDefinition {
        FlagDefinition::new()
    }

    fn args(&self) -> Option<ArgumentKind> {
        Some(ArgumentKind::Integer)
    }

    fn execute(
        &self,
        _: Flags,
        _: Option<Argument>,
        shell: &mut crate::shell::Shell,
    ) -> Result<CommandOutput, ShellError> {
        shell.active = false;
        Ok(CommandOutput(None))
    }
}
