//! The change directory command.

use crate::{
    FileSystem,
    commands::{Command, CommandOutput, flags::Flags},
    errors::ShellError,
};

#[derive(Default)]
pub struct ChangeDirectoryCommand;

impl Command for ChangeDirectoryCommand {
    fn name(&self) -> &'static str {
        "cd"
    }

    fn flags(&self) -> Flags {
        Flags::new()
    }

    fn execute(&self, _: &[String], _: &mut FileSystem) -> Result<CommandOutput, ShellError> {
        todo!()
    }
}
