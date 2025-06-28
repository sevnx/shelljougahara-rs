//! Create a directory.

use std::path::PathBuf;

use crate::{
    ShellError,
    commands::{
        Argument, CommandOutput, ExecutableCommand, Flags,
        args::{ArgumentKind, BasicArgument, BasicArgumentKind},
        flags::{FlagDefinition, FlagDefinitionBuilder},
    },
    errors::FileSystemError,
};

#[derive(Default, Clone, Copy)]
pub struct MakeDirectoryCommand;

impl ExecutableCommand for MakeDirectoryCommand {
    fn name(&self) -> &'static str {
        "mkdir"
    }

    fn flags(&self) -> FlagDefinition {
        FlagDefinitionBuilder::new().into_flag_definition()
    }

    fn args(&self) -> Option<ArgumentKind> {
        Some(ArgumentKind::Enumeration(BasicArgumentKind::String))
    }

    fn execute(
        &self,
        _: Flags,
        args: Option<Argument>,
        shell: &mut crate::shell::Shell,
    ) -> Result<CommandOutput, ShellError> {
        let error_builder = |path: &str, message: &str| {
            format!("mkdir: cannot create directory '{path}': {message}")
        };

        let mut current_session = shell.current_session.clone();
        let mut fs = shell.fs.clone();
        let mut error_messages = Vec::new();

        match args {
            Some(Argument::List(paths)) => {
                for path in paths {
                    let arg = match path {
                        BasicArgument::String(arg) => arg,
                        _ => return Err(ShellError::Internal("Invalid argument".to_string())),
                    };
                    let path = PathBuf::from(&arg);
                    if let Err(error) = current_session.create_directory(&mut fs, &path) {
                        match error {
                            ShellError::FileSystem(FileSystemError::EntryAlreadyExists(_)) => {
                                error_messages.push(error_builder(&arg, "File exists"));
                            }
                            ShellError::FileSystem(FileSystemError::DirectoryNotFound(_)) => {
                                error_messages
                                    .push(error_builder(&arg, "No such file or directory"));
                            }
                            _ => {
                                return Err(error);
                            }
                        }
                    }
                }
            }
            Some(_) => {
                return Err(ShellError::Internal("Invalid argument".to_string()));
            }
            None => {
                return Err(ShellError::Internal("missing operand".to_string()));
            }
        }

        if error_messages.is_empty() {
            Ok(CommandOutput(None))
        } else {
            Ok(CommandOutput(Some(error_messages.join("\n"))))
        }
    }
}
