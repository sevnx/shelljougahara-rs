//! The change directory command.

use std::path::PathBuf;

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
        let path = PathBuf::from(args.first().unwrap_or(&String::new()));
        match shell.current_session.change_directory(&shell.fs, &path) {
            Ok(()) => Ok(CommandOutput(None)),
            Err(ShellError::FileSystem(FileSystemError::DirectoryNotFound(_))) => Ok(
                CommandOutput(Some(format!("cd: {}: No such file or directory", args[0]))),
            ),
            Err(e) => Err(e),
        }
    }
}
