//! Remove a file or directory.

use crate::{
    ShellError,
    commands::{
        Argument, CommandOutput, ExecutableCommand, Flags, args::ArgumentKind,
        flags::FlagDefinition,
    },
};

#[derive(Default, Clone, Copy)]
pub struct RemoveCommand;

impl ExecutableCommand for RemoveCommand {
    fn name(&self) -> &'static str {
        "rm"
    }

    fn flags(&self) -> FlagDefinition {
        FlagDefinition::new()
    }

    fn args(&self) -> Option<ArgumentKind> {
        Some(ArgumentKind::Enumeration(Box::new(ArgumentKind::String)))
    }

    fn execute(
        &self,
        _: Flags,
        _: Option<Argument>,
        _: &mut crate::shell::Shell,
    ) -> Result<CommandOutput, ShellError> {
        todo!()
    }
}
