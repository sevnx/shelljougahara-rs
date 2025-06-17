//! Create a directory.

use std::path::PathBuf;

use crate::{
    ShellError,
    commands::{Command, CommandOutput, flags::Flags},
    errors::FileSystemError,
};

#[derive(Default)]
pub struct MakeDirectoryCommand;

impl Command for MakeDirectoryCommand {
    fn name(&self) -> &'static str {
        "mkdir"
    }

    fn flags(&self) -> Flags {
        Flags::new()
    }

    fn execute(
        &self,
        args: &[String],
        shell: &mut crate::shell::Shell,
    ) -> Result<CommandOutput, ShellError> {
        let error_builder = |path: &str, message: &str| {
            format!("mkdir: cannot create directory '{}': {}", path, message)
        };

        let mut current_session = shell.current_session.clone();
        let mut fs = shell.fs.clone();
        let mut error_messages = Vec::new();
        for arg in args {
            let path = PathBuf::from(arg);
            if let Err(error) = current_session.create_directory(&mut fs, &path) {
                match error {
                    ShellError::FileSystem(FileSystemError::EntryAlreadyExists(_)) => {
                        error_messages.push(error_builder(arg, "File exists"));
                    }
                    ShellError::FileSystem(FileSystemError::DirectoryNotFound(_)) => {
                        error_messages.push(error_builder(arg, "No such file or directory"));
                    }
                    _ => {
                        return Err(error);
                    }
                }
            }
        }
        if error_messages.is_empty() {
            Ok(CommandOutput(None))
        } else {
            Ok(CommandOutput(Some(error_messages.join("\n"))))
        }
    }
}
