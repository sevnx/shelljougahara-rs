//! The change directory command.

use std::path::PathBuf;

use crate::{
    commands::{
        Argument, CommandOutput, ExecutableCommand, Flags, args::ArgumentKind,
        flags::FlagDefinition,
    },
    errors::{FileSystemError, ShellError},
};

#[derive(Default, Clone, Copy)]
pub struct ChangeDirectoryCommand;

impl ExecutableCommand for ChangeDirectoryCommand {
    fn name(&self) -> &'static str {
        "cd"
    }

    fn flags(&self) -> FlagDefinition {
        FlagDefinition::new()
    }

    fn args(&self) -> Option<ArgumentKind> {
        Some(ArgumentKind::String)
    }

    fn execute(
        &self,
        _: Flags,
        args: Option<Argument>,
        shell: &mut crate::shell::Shell,
    ) -> Result<CommandOutput, ShellError> {
        let path = match args {
            Some(Argument::String(path)) => path,
            _ => return Err(ShellError::Internal("Invalid argument".to_string())),
        };
        let path = PathBuf::from(path);
        match shell.current_session.change_directory(&shell.fs, &path) {
            Ok(()) => Ok(CommandOutput(None)),
            Err(ShellError::FileSystem(FileSystemError::DirectoryNotFound(_))) => {
                Ok(CommandOutput(Some(format!(
                    "cd: {}: No such file or directory",
                    path.display()
                ))))
            }
            Err(e) => Err(e),
        }
    }
}
