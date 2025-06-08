//! The change directory command.

use crate::{
    FileSystem,
    commands::{Command, CommandOutput, flags::Flags},
    errors::{FileSystemError, ShellError},
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

    fn execute(
        &self,
        args: &[String],
        file_system: &mut FileSystem,
    ) -> Result<CommandOutput, ShellError> {
        let change_result = if args.is_empty() {
            file_system.change_directory("~")
        } else {
            file_system.change_directory(&args[0])
        };
        match change_result {
            Ok(_) => Ok(CommandOutput("".to_string())),
            Err(ShellError::FileSystem(FileSystemError::DirectoryNotFound(message))) => Ok(
                CommandOutput(format!("cd: {}: No such file or directory", message)),
            ),
            Err(e) => Err(e),
        }
    }
}
