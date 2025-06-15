//! The change directory command.

use crate::{
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
        shell: &mut crate::shell::Shell,
    ) -> Result<CommandOutput, ShellError> {
        let change_result = if args.is_empty() {
            shell.fs.change_directory("~")
        } else {
            shell.fs.change_directory(&args[0])
        };
        match change_result {
            Ok(_) => Ok(CommandOutput(None)),
            Err(ShellError::FileSystem(FileSystemError::DirectoryNotFound(_))) => Ok(
                CommandOutput(Some(format!("cd: {}: No such file or directory", args[0]))),
            ),
            Err(e) => Err(e),
        }
    }
}
