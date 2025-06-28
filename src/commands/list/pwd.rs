//! The print working directory command.

use crate::{
    commands::{
        Argument, CommandOutput, ExecutableCommand, Flags,
        args::ArgumentKind,
        flags::{FlagDefinition, FlagDefinitionBuilder},
    },
    errors::ShellError,
};

#[derive(Default, Clone, Copy)]
pub struct PwdCommand;

impl ExecutableCommand for PwdCommand {
    fn name(&self) -> &'static str {
        "pwd"
    }

    fn flags(&self) -> FlagDefinition {
        // TODO: Maybe add `-L` and `-P` flags.
        FlagDefinitionBuilder::new().into_flag_definition()
    }

    fn args(&self) -> Option<ArgumentKind> {
        None
    }

    fn execute(
        &self,
        _: Flags,
        _: Option<Argument>,
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
