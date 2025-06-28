//! The history command, printing the history of commands.

use crate::{
    commands::{
        Argument, CommandOutput, ExecutableCommand, Flags,
        args::{ArgumentKind, BasicArgument, BasicArgumentKind},
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
        Some(ArgumentKind::Basic(BasicArgumentKind::Integer))
    }

    fn execute(
        &self,
        _: Flags,
        args: Option<Argument>,
        shell: &mut crate::shell::Shell,
    ) -> Result<CommandOutput, ShellError> {
        let history = shell.current_session.get_history();

        let history_iter = history.iter().enumerate();

        let history = match args {
            Some(Argument::Basic(BasicArgument::Integer(limit))) => {
                history_iter.take(limit as usize).collect::<Vec<_>>()
            }
            Some(_) => return Err(ShellError::Internal("Invalid argument".to_string())),
            None => history_iter.collect::<Vec<_>>(),
        };

        let history = history
            .into_iter()
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
