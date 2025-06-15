//! The history command, printing the history of commands.

use crate::{
    commands::{Command, CommandOutput, flags::Flags},
    errors::ShellError,
};

#[derive(Default)]
pub struct HistoryCommand;

impl Command for HistoryCommand {
    fn name(&self) -> &'static str {
        "history"
    }

    fn flags(&self) -> Flags {
        Flags::new()
    }

    fn execute(
        &self,
        _: &[String],
        shell: &mut crate::shell::Shell,
    ) -> Result<CommandOutput, ShellError> {
        let history = shell
            .current_session
            .get_history()
            .iter()
            .enumerate()
            .map(|(index, command)| format!("{:>5} {}", index + 1, command))
            .collect::<Vec<String>>()
            .join("\n");

        Ok(CommandOutput(Some(history)))
    }
}
