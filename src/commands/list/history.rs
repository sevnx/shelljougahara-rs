//! The history command, printing the history of commands.

use crate::{
    commands::{
        Argument, CommandOutput, ExecutableCommand, Flags, args::ArgumentKind,
        flags::FlagDefinition,
    },
    errors::ShellError,
};

#[derive(Default, Clone, Copy)]
pub struct HistoryCommand;

impl ExecutableCommand for HistoryCommand {
    fn name(&self) -> &'static str {
        "history"
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
        args: Option<Argument>,
        shell: &mut crate::shell::Shell,
    ) -> Result<CommandOutput, ShellError> {
        let limit = match args {
            Some(Argument::Integer(limit)) => limit,
            _ => return Err(ShellError::Internal("Invalid argument".to_string())),
        };

        let history = shell
            .current_session
            .get_history()
            .iter()
            .enumerate()
            .take(limit as usize)
            .map(|(index, command)| format!("{:>5} {}", index + 1, command))
            .collect::<Vec<String>>()
            .join("\n");

        Ok(CommandOutput(if history.is_empty() {
            None
        } else {
            Some(history)
        }))
    }
}
